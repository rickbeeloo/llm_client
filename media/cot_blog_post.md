### An example cascade: CoT reasoning

This is in progress. 

An example of a cascade workflow is the [one round reasoning workflow](./src/workflows/reason/one_round.rs).

First we insert a guidance round the the user message that defines the workflow, followed by a faux 'guidance' response assistant message.

```rust
flow.new_round(
"A request will be provided. Think out loud about the request. State the arguments before arriving at a conclusion with, 'Therefore, we can conclude:...', and finish with a solution by saying, 'Thus, the solution...'. With no yapping.").add_guidance_step(
&StepConfig {
    ..StepConfig::default()
},
"'no yapping' refers to a design principle or behavior where the AI model provides direct, concise responses without unnecessary verbosity or filler content. Therefore, we can conclude: The user would like to get straight to the point. Thus, the solution is to to resolve the request as efficiently as possible.",
);
```

Then we create our inference round, starting with the initial 'reasoning' step. We build the user generated dynamic prompt as the task, set the prefix, stop word, and constrain generation to N number of sentences. In this step the LLM 'thinks' wth CoT reasoning towards the answer.

```rust
let step_config = StepConfig {
    step_prefix: Some("Thinking out loud about the users request...".to_string()),
    stop_word_done: "Therefore, we can conclude".to_string(),
    grammar: SentencesPrimitive::default()
        .min_count(1)
        .max_count(3)
        .grammar(),
    ..StepConfig::default()
};
flow.new_round(self.build_task()?).add_inference_step(&step_config);
```

Next is our solution step where we insert a prefix that states the type of primitive result we want and if it can be 'None'. This gives the LLM an opportunity to provide an unconstrained answer.

```rust
let step_config = StepConfig {
    step_prefix: Some(format!(
        "The user requested a conclusion of {}. Therefore, we can conclude:",
        self.primitive.solution_description(self.result_can_be_none),
    )),
    stop_word_done: "Thus, the solution".to_string(),
    grammar: SentencesPrimitive::default()
        .min_count(1)
        .max_count(2)
        .grammar(),

    ..StepConfig::default()
};
flow.last_round()?.add_inference_step(&step_config);
```

Optionally, we add a guidance step that restates the instructions. Often LLMs suffer with following instructions in long context, so this restates what the outcome should be.

```rust
let step_config = StepConfig {
    step_prefix: None,
    grammar: SentencesPrimitive::default().grammar(),
    ..StepConfig::default()
};
flow.last_round()?
    .add_guidance_step(&step_config, format!("The user's original request was '{}'.", &instructions,));
```

In the final step we extract the result in the format of the requested primitive. The output is constrained here, and is then parsed into the primitive.

```rust
let step_config = StepConfig {
    step_prefix: Some(format!(
    "Thus, the {} solution to the user's request is:",
    self.primitive.type_description(self.result_can_be_none),
)),
    stop_word_null_result: self
        .primitive
        .stop_word_result_is_none(self.result_can_be_none),
    grammar: self.primitive.grammar(),
    ..StepConfig::default()
};
flow.last_round()?.add_inference_step(&step_config);
```

Finally, we run the workflow. 

```rust
flow.run_all_rounds(&mut self.base_req).await?;
```

In this example the work flow is run linearly as built, but it's also possible to run dynamic workflows where each step is ran one at a time and the behavior of the workflow can be dynamic based on the outcome of that step. See [extract_urls](./examples/extract_urls.rs) for an example of this.
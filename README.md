<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
<!-- [![LinkedIn][linkedin-shield]][linkedin-url] -->




<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->

## A rust interface for the OpenAI API and Llama.cpp ./server API 

* A unified API for testing and integrating OpenAI and HuggingFace LLM models.
* Load models from HuggingFace with just a URL.
* Uses <a href="https://github.com/ggerganov/llama.cpp/tree/master/examples/server">Llama.cpp server API</a> rather than bindings, so as long as the Llama.cpp server API remains stable this project will remain usable.
* Prebuilt agents - not chatbots - to unlock the true power of LLMs.

### Easily switch between models and APIs
```
// Use an OpenAI model
let llm_definition = LlmDefinition::OpenAiLlm(OpenAiLlmModels::Gpt35Turbo)
```
```
// Or use a model from hugging face
let zephyr_7b_chat = LlamaLlmModel::new(
    url, 
    LlamaPromptFormat::Mistral7BChat, 
    Some(2000), // Max tokens for model AKA context size
);

let response = basic_text_gen::generate(
        &LlmDefinition::LlamaLlm(zephyr_7b_chat),
        Some("Howdy!"),
    )
    .await?;
eprintln!(response)
```
### Get deterministic responses from LLMs
```
if !boolean_classifier::classify(
        llm_definition,
        Some(hopefully_a_list),
        Some("Is the attached feature a list of content split into discrete entries?"),
    )
    .await?
    {
        panic!("{}, was not properly split into a list!", hopefully_a_list)
    }

```
### Dependencies 
<a href="https://github.com/64bit/async-openai">async-openai</a> is used to interact with the OpenAI API. A modifed version of the async-openai crate is used for the Llama.cpp server. If you just need an OpenAI API interface, I suggest using the async-openai crate.

<a href="https://github.com/huggingface/hf-hub"> Hugging Face's rust client</a> is used for model downloads from the huggingface hub. 

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

### Step-by-step guide
1. Clone repo:
```
git clone https://github.com/ShelbyJenkins/llm_client.git
cd llm_client
```
2. Optional: build devcontainer from `llm_client/.devcontainer/devcontainer.json`
3. Add llama.cpp:
```
git submodule init 
git submodule update
```
4. Build llama.cpp (<a href="https://github.com/ggerganov/llama.cpp">instructions here</a>):
  ```
  // Example build for nvidia gpus
  cd llm_client/src/providers/llama_cpp/llama_cpp
  make LLAMA_CUBLAS=1
  ```
5. Test llama.cpp ./server
```
cargo run -p llm_client --bin server_runner start --model_url "https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/blob/main/mistral-7b-instruct-v0.2.Q8_0.gguf"
```
This will download and load the given model, and then start the server.

When you see `llama server listening at http://localhost:8080`, you can load the llama.cpp UI in your browser.

Stop the server with `cargo run -p llm_client --bin server_runner stop`.

6. Using OpenAi: Add a `.env` file in the llm_client dir with the var `OPENAI_API_KEY=<key>`


### Examples

<a href="llm_client/examples/basic_text_gen.rs">A quick example of interacting with the provided agents</a>

<a href="llm_client/examples/llm_client.rs">Interacting with the llm_client directly</a>

<!-- ROADMAP -->
## Roadmap

* Automate starting the llama.cpp with specified model
* Handle the various prompt formats of LLM models more gracefully
* Unit tests
* Add additional classifier agents:
    * many from many
    * one from many
* Implement all openai functionality with llama.cpp
* More external apis (claude/etc)
  
<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

I would love any help adding features!


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Shelby Jenkins - Here or Linkedin 


<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/ShelbyJenkins/llm_client.svg?style=for-the-badge
[contributors-url]: https://github.com/ShelbyJenkins/shelby-as-a-service/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/ShelbyJenkins/llm_client.svg?style=for-the-badge
[forks-url]: https://github.com/ShelbyJenkins/shelby-as-a-service/network/members
[stars-shield]: https://img.shields.io/github/stars/ShelbyJenkins/llm_client.svg?style=for-the-badge
[stars-url]: https://github.com/ShelbyJenkins/shelby-as-a-service/stargazers
[issues-shield]: https://img.shields.io/github/issues/ShelbyJenkins/llm_client.svg?style=for-the-badge
[issues-url]: https://github.com/ShelbyJenkins/shelby-as-a-service/issues
[license-shield]: https://img.shields.io/github/license/ShelbyJenkins/llm_client.svg?style=for-the-badge
[license-url]: https://github.com/ShelbyJenkins/shelby-as-a-service/blob/master/LICENSE.txt
<!-- [linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com -->


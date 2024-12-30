<a id="readme-top"></a>
<br />
<div align="center">

  <a href="https://github.com/BrainFlight/Synapse"><img width="140px" src="./docs/images/logo.png"></a>

  <h3 align="center">Synapse</h3>

  <p align="center">
    VLA (Vision-Language-Action) model serving platform that helps you manage robot instruction prompts, RAG, and waypoint metadata.
    <br />
    <a href="https://github.com/BrainFlight/Synapse"><strong>Explore the docs »</strong></a>
    <br />
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<!-- <details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details> -->


## About The Project

Embodied AI is an emerging and very promising research field but at the moment, it is difficult to break into. We aim to improve the accessibility of large multi-modal language models that convert natural language instructions into serialized robot actions. Synapse is our first step in addressing this as it provides a low-latency inference API for VLA (Vision-Language-Action) models and LLMs catered towards robotics use cases. 

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Getting Started


### Prerequisites

Get API Keys

`.env` file:
```sh
COHERE_API_KEY=""
```

### Installation

```sh
cargo build
```

### API and Swagger Docs

1. Run the API

2. Visit http://localhost:8080/docs

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Roadmap

- Enable Compatibility with OpenAI, Anthropic, HuggingFace, and custom models
- Integrate gRPC communication with onboard robot system
- Enable high-throughput image and point-cloud processing for robots with LiDAR, radar, or stereo cameras

<p align="right">(<a href="#readme-top">back to top</a>)</p>

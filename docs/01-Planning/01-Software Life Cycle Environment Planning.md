# Software Life Cycle Environment Planning

## Software Life Cycle

### Software Life Cycle Definition

The software life cycle for NeoExplorer follows the V-Model, a sequential development process that emphasizes verification and validation at each stage. This model ensures that every development phase is directly linked to a corresponding testing phase, promoting a thorough and systematic approach to software development.

### V-Model Diagram for NeoExplorer

Below is a representation of the V-Model tailored for NeoExplorer:

```txt
Planning
    \
Requirements Analysis     <-->            User Acceptance Testing
      \                                          /
   System Design          <-->            System Testing
         \                                     /
   Detailed Design        <-->          Integration Testing
           \                                 /
           Coding         <-->         Unit Testing
```

### Description of V-Model Stages

1. **Planning**: Define the project goals, create a roadmap, and allocate resources.
2. **Requirements Analysis**: Gather and document all requirements.
3. **System Design**: Develop high-level architecture and design.
4. **Detailed Design**: Create detailed component-level designs.
5. **Implementation (Coding)**: Write and integrate the code.
6. **Unit Testing**: Test individual units for functionality.
7. **Integration Testing**: Ensure integrated modules function together.
8. **System Testing**: Validate the complete system.
9. **User Acceptance Testing (UAT)**: Ensure the system meets user requirements.
10. **Deployment**: Deploy the system to the production environment.
11. **Maintenance**: Provide ongoing support and updates.

### Software Life Cycle Processes

#### Planning

- **Objective**: Define project goals, scope, and objectives.
- **Activities**: Create a detailed project roadmap and timeline, identify stakeholders, allocate resources.
- **Output**: [Project Plan Document](/docs/01-Planning/05-Software%20Planning%20Roadmap.md).

#### Requirements Analysis

- **Objective**: Gather and document functional and non-functional requirements.
- **Activities**: Conduct stakeholder interviews, develop use cases, create requirement specifications.
- **Output**: [Software Requirements Specification](/docs/02-Requirements/03-Software%20Requirement%20Specification.md).

#### System Design

- **Objective**: Design the overall system architecture and high-level design components.
- **Activities**: Define system modules, data flow diagrams, and database schema.
- **Output**: [System Design Document](/docs/02-Requirements/02-System%20Design%20Document.md).

#### Detailed Design

- **Objective**: Develop detailed designs for each system component.
- **Activities**: Create detailed design specifications, UI mockups, and component interfaces.
- **Output**: Detailed Design Document (This will documented inside source code itself).

#### Implementation (Coding)

- **Objective**: Write and integrate code for all system components.
- **Activities**: Develop code according to design specifications, perform unit testing, integrate modules.
- **Output**: [Source Code](/src/).

#### Integration Testing

- **Objective**: Verify that integrated modules work together as expected.
- **Activities**: Develop integration test plans, execute test cases, document test results.
- **Output**: Integration Test Report.

#### System Testing

- **Objective**: Validate the complete system's functionality and performance.
- **Activities**: Develop system test plans, execute test cases, perform performance testing.
- **Output**: System Test Report.

#### User Acceptance Testing (UAT)

- **Objective**: Ensure the system meets user requirements and is ready for deployment.
- **Activities**: Develop UAT plans, conduct user testing sessions, gather feedback.
- **Output**: UAT Report.

#### Deployment

- **Objective**: Deploy the system to the production environment.
- **Activities**: Prepare deployment plan, configure the production environment, deploy the system.
- **Output**: [Deployment Plan](docs/01-Planning/03-Software Configuration Management.md).

#### Maintenance

- **Objective**: Provide ongoing support and updates for the system.
- **Activities**: Monitor system performance, fix bugs, implement updates.
- **Output**: Maintenance Logs, Updated System.

### Transition Criteria Between Processes

- **Planning to Requirements Analysis**: Approval of the project plan and roadmap.
- **Requirements Analysis to System Design**: Finalization and sign-off on the SRS.
- **System Design to Detailed Design**: Approval of the System Design Document.
- **Detailed Design to Implementation**: Finalization of the Detailed Design Document.
- **Implementation to Integration Testing**: Completion of coding and unit testing.
- **Integration Testing to System Testing**: Successful integration test report.
- **System Testing to UAT**: Successful system test report.
- **UAT to Deployment**: Positive user feedback and UAT report.
- **Deployment to Maintenance**: Successful deployment and initial user feedback.

## Software Development Environment

### Tech Stack Considerations

#### Choice of Editor

- **VSCode**: Any IDE can be chosen but for our development we chose VSCode mainly due to wide support, single editor and do the most jobs with the help of 3rd party extensions.
- **Extensions**:
  - **Rust**:
    1. [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
    2. [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    3. [Rust Doc Viewer](https://marketplace.visualstudio.com/items?itemName=JScearcy.rust-doc-viewer)
    4. [Rust Test Explorer](https://marketplace.visualstudio.com/items?itemName=swellaby.vscode-rust-test-adapter)
  - **Tauri**:
    1. [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - **Documentation**:
    1. [Draw.io Integration](https://marketplace.visualstudio.com/items?itemName=hediet.vscode-drawio)
    2. [Markdownlint](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint)
  - **Optional**
    1. [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
    2. [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
    3. [Git Graph](https://marketplace.visualstudio.com/items?itemName=mhutchie.git-graph)
    4. [Markdown All in One](https://marketplace.visualstudio.com/items?itemName=yzhang.markdown-all-in-one)
    5. [Markdown+Math](https://marketplace.visualstudio.com/items?itemName=goessner.mdmath)
    6. [Path Intellisense](https://marketplace.visualstudio.com/items?itemName=christian-kohler.path-intellisense)
    7. [Better Comments](https://marketplace.visualstudio.com/items?itemName=AmirHA.better-comments-2)

#### Choice of Documentation

- **Markdown**: For easy-to-write and maintain documentation.
- **Draw.io**: For creating design diagrams and flowcharts.

#### Choice of Database

- **SQLite**: Lightweight and easy to integrate for storing settings, preferences, and user data.

#### Choice of Frontend Language

- **Tauri**: Chosen for its efficiency, ease of use, and ability to create responsive and modern UIs.

#### Choice of Backend Language

- **Rust**: Chosen for its performance, memory safety, and modern features suitable for developing a robust file explorer.

### Choice of Test Tool

- **Cargo Test**: For running unit tests and integration tests within the Rust environment.
- **Manual Testing**: For user acceptance testing and verifying real-world usability.

### Setting up the environment

#### Setting up Rust

- Download the [Rust installer](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
- Run the program and follow the onscreen instructions
- To check installation , you can run this in your terminal of choice: `cargo --version`
- For more information read the [guide](https://www.rust-lang.org/learn/get-started)

#### Setting up Tauri

- Download the [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) installer and open it to begin installation.
- During installation check the “Desktop development with C++” option.
- WebView 2 is already installed on Windows 10 or else [download here](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section)
- To install Tauri Run the following commands
  - `cargo install create-tauri-app`
  - `cargo create-tauri-app --beta`
    - Project Name : NeoExplorer
    - Package Name : neoexplorer
    - Package Manager : npm
    - Front End : SvelteKit (Javascript)
  - `cd NeoExplorer`
  - `npm install`
  - `npm run tauri dev`

## Configuration Management Environment

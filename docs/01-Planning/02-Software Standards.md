# Software Standards

## Software Requirement Standards

### Software Requirement Templates

```Markdown
# NXP: Software Requirement Specification

## NXP-INF: Introduction

## NXP-REF: Reference Documents

### NXP-REF-INT: Internal Documents

### NXP-REF-EXT: External Documents

## NXP-REQ: System Requirements

### NXP-REQ-MODULE: Module

#### NXP-REQ-MODULE-SUBMODULE: Submodule

##### NXP-REQ-MODULE-SUBMODULE-UIX: UI/UX Requirements

###### NXP-REQ-MODULE-SUBMODULE-UIX-H: High Priority Requirement

1. **NXP-REQ-MODULE-SUBMODULE-UIX-H-1: Requirement Title - 1**
   - **Description :** Requirement Description
   - **Additional Information :** Requirement additional information
2. **NXP-REQ-MODULE-SUBMODULE-UIX-H-2: Requirement Title - 2**
3. **NXP-REQ-MODULE-SUBMODULE-UIX-H-3: Requirement Title - 3**

###### NXP-REQ-MODULE-SUBMODULE-UIX-M: Medium Priority Requirement

###### NXP-REQ-MODULE-SUBMODULE-UIX-L: Low Priority Requirement

##### NXP-REQ-MODULE-SUBMODULE-FUN: Functional Requirements

##### NXP-REQ-MODULE-SUBMODULE-NFR: Non-Functional Requirements

## NXP-CRM: Cross Reference Matrix

## NXP-APN: Appendices

```

## Software Development Standards

## Software Design Standards

### Software Coding Templates

```Rust
/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: <package_name>
 * File Name: <file_name>.rs
 * Author: <Author Name>
 * Description: <Brief description of the file and its purpose>
 ******************************************************************************/
 
/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries
use std::<library>;

// External Crates
use <crate_name>::<module>;

// Internal Modules
mod <module_name>;

/******************************************************************************
 * Constants:
******************************************************************************/

const <CONSTANT_NAME>: <type> = <value>;

/******************************************************************************
 * Structures and Enums:
 ******************************************************************************/

enum <EnumName> {
    <Variant1>,
    <Variant2>,
    // ...
}

struct <StructName> {
    <field_name>: <field_type>,
    // ...
}

/******************************************************************************
 * Implementations:
 ******************************************************************************/

impl <StructName> {
    fn <method_name>(&self, <param>: <type>) -> <return_type> {
        // ...
    }
}

/******************************************************************************
 * Functions:
 ******************************************************************************/

fn <function_name>(<arg_name>: <type>) -> <return_type> {
    // ...
}

```

### Software Coding Standards

## Software Verification Standards

### Software Test Plan Standards

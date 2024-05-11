# Macronutritional Counter

<!--toc:start-->

A simple terminal program to track, record, and view macronutritional vectors.
i.e. calories, fat, carbohydrates, and protein.

- [Usage](#usage)
- [Operations](#operations)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
<!--toc:end-->

## Usage

![menu](https://github.com/Vallen217/macro_counter/assets/94763660/f1c6e518-c2f3-4f16-b72b-c892b7787a66)


macro_counter creates directries & file based on the current date (e.g. `2023/09/27.txt`)
in the directory `$HOME/Documents/Health/Macronutritional_Intake`.
Which are used as the basis to track & record macro data.

## Operations

### (mf) - Modify file

![mf](https://github.com/Vallen217/macro_counter/assets/94763660/514c9d74-7ca5-4670-9b1f-7a6b230da084)


### (dr) - Display the most recent, non-current file

![dr](https://github.com/Vallen217/macro_counter/assets/94763660/5bec74e0-2a75-4cbe-b8ca-9d0feb8725a2)


### (dpf) - Display previous files

![dpf](https://github.com/Vallen217/macro_counter/assets/94763660/486fcdc1-88c8-4fcc-bf6a-c1ee3e862f04)


### (dpm) - Display previous month's aggregated data

![dpm](https://github.com/Vallen217/macro_counter/assets/94763660/f45365c7-e211-49cf-819f-8857103c2618)


### (dp#) - Display aggregated data from the previous # files

![dp#](https://github.com/Vallen217/macro_counter/assets/94763660/0e7226e6-8131-4399-b18c-e5a4a86ed74e)


### (df) - Display the current file

![df](https://github.com/Vallen217/macro_counter/assets/94763660/b450a65f-69a5-4dc3-b53d-90322af948f6)

    
### (dm) - Display the current month's aggregated data

![dm](https://github.com/Vallen217/macro_counter/assets/94763660/d8625776-9970-47f8-8ef7-15e47a486827)


## (pd) - Predefined meals

![pd_menu](https://github.com/Vallen217/macro_counter/assets/94763660/93ff6d51-168c-4ac1-9cdc-f7f991d0e4d2)


### (cf) - create a new predefined meal file & (mf) - Modify predefined meal files

![pd_cf_mf](https://github.com/Vallen217/macro_counter/assets/94763660/69ee1999-fd77-4b64-846c-d2fef959c63f)


### (rf) - Remove the latest predefined file

![pd_rf](https://github.com/Vallen217/macro_counter/assets/94763660/3bdb99cd-7b85-4ee0-8051-769d850300db)


### (m#) - Append predefined meal m# to the current file

![m#](https://github.com/Vallen217/macro_counter/assets/94763660/e62a32b6-14cf-4fc8-a2d6-87a4ed0254d0)


## Prerequisites

The latest stable [Rust](https://ww.rust-lang.org) compiler.

## Installation

Download the repository with either of the below:

1. `cargo install --git https://github.com/Vallen217/macro_counter`

2. `git clone https://github.com/Vallen217/macro_counter` \
   #Note: In this case you'll need to run a simple setup script to run `mctr.sh`
   without typing out the absolute path every iteration: \
   `bash <path_to_clone>/scripts/mctr_setup.sh `

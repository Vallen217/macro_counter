# Macronutritional Counter

<!--toc:start-->

A simple terminal program to track, record, and view macronutritional vectors.
i.e. calories, fat, carbohydrates, and protein.

- [Usage](#usage)
- [Operations](#operations)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Setup](#setup)
<!--toc:end-->

## Usage
Run `mctr` in a terminal:

    (mf)  - Modify file
    (dpf) - Display previous files
    (dpm) - Display previous monthly data
    (df)  - Display file
    (dm)  - Display monthly data
    (pd)  - Predefined meals
    (m#)  - Append predefined meal m#
    (q)   - Quit the program
    Operation:

macro_counter creates directries & file based on the current date (e.g. `2023/09/27.txt`)
in the directory `$HOME/Documents/Health/Macronutritional_Intake`.
Which are used as the basis to track & record macro data.

## Operations

### (mf) - Modify file

Add/remove data to/from a file.

    Operation:
    mf

    (rl#)  - Remove the last 'n' file entry lines
    (rlq#) - Remove the last 'n' file entry lines and quit
    (.)    - Repeat the last data entry line
    (q)    - Quit the 'mf' loop
    (Press any key to continue)


    Caloire:
    200

    Fat:
    5

    Carb:
    24

    Protein:
    16

    (Press enter to continue)
    Operation:
    q

    Cal:        Fat:        Carb:       Protein:
    200         5g           24g          16g

    Total Amounts & Relative Percentages:
    200         5g          24g         16g
                11.1%       53.3%       35.6%

### (df) - Display file

Displays data recorded in a file.

    Operation:
    df

    Cal:        Fat:        Carb:       Protein:
    200         5g           24g          16g

    Total Amounts & Relative Percentages:
    200         5g          24g         16g
                11.1%       53.3%       35.6%

### (dm) - Display monthl data

Display aggregated data from every file in the monthly directory.
Note: in this example only 4 files are present in the monthly directory.

    Operation:
    dm


    Cal:        Fat:        Carb:       Protein:

    Contemporary monthly total amounts:
    2960        90g         439g        229g

    Mean daily amounts:
    740.0       22.5g       109.8       57.2g

    Mean daily relative percentages:
                11.9%       57.9%       30.2%

### (dpf) - Display previous files

    Operation:
    dpf

    Enter a relative path from:
    $HOME/Documents/Health/Macronutritional_Intake/2023-09
    2023-09

    $HOME/Documents/Health/Macronutritional_Intake/2023-09/24.txt
    $HOME/Documents/Health/Macronutritional_Intake/2023-09/25.txt
    $HOME/Documents/Health/Macronutritional_Intake/2023-09/26.txt
    $HOME/Documents/Health/Macronutritional_Intake/2023-09/27.txt
    25

    Cal:        Fat:        Carb:       Protein:
    200         5g           24g          16g
    280         2g           55g          10g

    Total Amounts & Relative Percentages:
    480         7g          79g         26g
                6.2%        70.5%       23.2%

### (dpm) - Display previous monthly data

    Operation:
    dpm

    Enter a relative path from:
    $HOME/Documents/Health/Macronutritional_Intake/2023-09
    2023-09


    Cal:        Fat:        Carb:       Protein:

    Contemporary monthly total amounts:
    2960        90g         439g        229g

    Mean daily amounts:
    740.0       22.5g       109.8       57.2g

    Mean daily relative percentages:
                11.9%       57.9%       30.2%

### (pd) - Predefined meals

If you find yourself frequently entering in an identical set of macro data e.g.
for breakfast you frequently have 2 granola bars & 2 bagel,
instead of enter data from each item separateley each day you could use predefined meals.

Which creates a new file named `m#` (where # is the existing number of predefined meals)
and add the data you'd usually add to the daily files in `$HOME/Documents/Health/Macronutritional_Intake`
which can then be called with `m1` when running the `mf` operation,
to append whatever's in predefined meal file to the current daily file.

Access point for creating/modifying predefined files
All operations available are the same as the aforementions.

    Operaton:
    pd


    (cf)  - Create new predefined meal
    (mf)  - Modify predefined meal
    (df)  - Dispaly predefined meals
    (q)   - Quit the 'pd' loop

    Operation:
    df

    $HOME/Documents/Health/Predefined_Meals/m1.txt
    m1

    Cal:        Fat:        Carb:       Protein:
    180         6g          21g         12g
    180         6g          21g         12g
    280         2g          55g         10g
    280         2g          55g         10g

    Total Amounts & Relative Percentages:
    920         16g         21g         12g
                7.5%        71.7%       20.8%

### (m#) - Append predefined meal m#

    Operation:
    m1

    Cal:        Fat:        Carb:       Protein:
    200         5g          24g         16g
    180         6g          21g         12g
    180         6g          21g         12g
    280         2g          55g         10g
    280         2g          55g         10g

    Total Amounts & Relative Percentages:
    1120        21g         176g        60g
                8.2%        68.5%       23.3%

### (q) - Quit the program

self explanitory.

    Operation:
    q

## Prerequisites
the latest stable [Rust](https://ww.rust-lang.org) compiler.

## Installation

```
cargo install --git https://github.com/Vallen217/macro_counter
```

## Setup
You'll need to create the two directories which will store files generated by macro_counter.
```
mkdir -p "$HOME/Documents/Health/Macronutritional_Intake"
mkdir -p "$HOME/Documetns/Health/Predefined_Meals"
```

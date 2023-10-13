# Coldchain

Coldchain is designed to work with mSupply's Coldchain Application.
https://docs.msupply.foundation/coldchain/introduction/

## Installation

Configure notify to use a postgres database that's receiving cold chain data from mSupply.

## Logic

Here's the state diagram summarising the logic of the cold chain alerts.

```mermaid
flowchart TD
    Start{Has State Changed?}
    Start --> |Yes| NewState
    Start --> |No| SameState
    SameState --> |CheckRepeatInterval|CheckRepeatInterval
    CheckRepeatInterval{Repeat Message due?}
    NewState{New State}
    NewState -->|Ok| D[Send Ok Confirmation]
    NewState -->|High| E[Send High Temp]
    NewState -->|Low| F[Send Low Temp]
    NewState -->|No Data| G[Send No Data]
    CheckRepeatInterval --> |Yes| X[Send reminder message]
    CheckRepeatInterval --> |No| Y[Do nothing]
```

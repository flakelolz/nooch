# Setup
- [x] setup dear imgui
- [x] sprite and UI render targets
- [x] increase screen size
- [x] game scales with screen size
- [ ] setup flecs explorer with reflection


# Gameplay
- [x] World
- [x] input handling with bitwise operations
- [x] draw characters
- [x] Read state from data file
- [x] state machine
- [x] physics system
- [x] input buffer
- [x] second player
- [x] Character data
- [x] State transition utilities
- [x] World to screen utilities
- [x] Display current State
- [x] Pause and frame advance
- [x] Animation system
- [x] Allow attack to attack transition without an Idle frame in between
- [x] Incorporate every standing and crouching attack
- [x] Position modifiers on states
- [ ] Chain attack modifier
- [x] Buffer system
- [x] Flip logic and buffer system when flipping
- [x] Dash lockout after walking
- [ ] Fix attack input buffer for chain attacks
- [x] Implement jump states
- [x] Dash lockout
- [ ] Screen size options
- [x] Implement flipping logic
- [x] Collision system
- [x] Reaction system
- [ ] Implement blocking
- [ ] Implement all hit reaction states
- [x] Make an editor for character data
- [x] Be able to see changes instantly while paused
- [x] synchronize changes between both players if they're the same character
- [x] Change font
- [x] Push logic
- [x] walls
- [x] Apply knock-back to attacker when defender is cornered
- [ ] Pull character on cross-up hit
- [ ] Background
- [ ] Camera
- [ ] Proximity normals
- [ ] Target Combo
- [ ] Special moves
- [ ] Special cancels
- [ ] Fix direction of motion when crossing-up the opponent in the air
- [ ] Implement all knockdown states

# Coding

- [ ] Maybe change data from JSON to RON files
- [ ] Change keys for ActionData and AnimationData hashmaps to use an enum instead of a String
- [ ] Logging crate

## Dash should fail

- [ ] *4* > *5* > *6* (walk back and forth repeatedly)
- [ ] *4...* (walking) ->  *5* > *4*
- [ ] *1...* (crouching) ->  *4* > *5* > *4*;
- [ ] *5...* (standing) -> *6* > *5* > *2* > *5* > *6*
- [ ] *5...* (standing) -> *6* > *5* > *3* > *6*

## Dash should work

- [ ] *4...* (walk back) ->  *5* > *4* > *5* > *4*
- [ ] *1...* (crouching) ->  *5* > *4* > *5* > *4*
- [ ] *5...* (standing) -> *3* > *5* > *6*
- [ ] *5...* (standing) -> *1* > *5* > *4*
- [ ] *5...* (standing) -> *4* > *6* > *5* > *6*
- [ ] *5...* (standing) -> *6* > *3* > *5* > *6*


## Motions

I will most likely need to read them backwards but also have priority between motions

- [ ] *6*... -> *2* > *3* > *6* => DP
- [ ] *6*... -> *4* > *1* > *2* > *3* > *6* => QCF

This needs to have a bigger input leniency
- [ ] *6*... -> *2* > *cr mk* > *3* > *lp* > => Cr MK xx DP

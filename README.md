# 🧙 The Number Wizard 
The classic number guessing game with a magical twist. 

Who said that coding a number guessing game should be dull? Meet the **Number Wizard**, who will give you hot/cold feedback and maybe even a desperate hint! 

## 🎮 How to Play
The Number Wizard thinks of a number between **0 and 99**. You have **10 attempts** to find it!

Each guess earns a **temperature rating** based on how close you are.

### Temperature Scale: 
- 🥶 **Freezing** - Way off (50+ away)
- ❄️ **Cold** - Not close (21-50 away)
- 😐 **Cool** - Getting there (11-20 away) 
- 🙂 **Warm** - Close (6-10 away)
- 🥵 **Hot** - Very close (3-5 away)
- 🔥 **Burning** - Almost there (1-2 away)
- 🎆 **ON FIRE!** - Perfect! You found it!

Your guess history is displayed in matching colours.

## 🚀 Quick Start 
```console 
# Clone and run 
git clone https://github.com/flloaers/number-wizard.git
cd number-wizard 
cargo run
```

## 📦 Dependencies
- ``rand = "0.9.3"`` - For random number generation
- ``colored = "3.0.0"`` - For colourful terminal output

## 📖 Concepts Explored 
This project was built as a Rust learning project exploring:
- `Enum` with methods 
- `Struct` organisation and `impl` blocks 
- Error handling with `Result<T, E>`
- Pattern matching 
- External crate integration (`rand` and  `colored`)

## 🤝 Contributing
This is a **learning project**, so feedback is appreciated and contributions are welcome! Feel free to fork the repository, open an issue, or submit a pull request. 

## 📜  License
This repository is licensed under the [MIT License](https://github.com/flloaers/renshuu-activity-tracker?tab=MIT-1-ov-file).

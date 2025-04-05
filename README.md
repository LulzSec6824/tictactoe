# 🎮 Tic-Tac-Toe Game in Rust 🦀

🌟 A classic Tic-Tac-Toe game implemented in Rust with an unbeatable AI opponent using the minimax algorithm!

## 📋 Table of Contents
- 🚀 [Features](#✨-features)
- 🛠️ [Installation](#🚀-installation)
- 🎮 [How to Play](#🎯-how-to-play)
- 📜 [Game Rules](#📝-game-rules)
- 🤖 [About the AI](#🤖-about-the-ai)
- 📄 [License](#📜-license)
- 🤝 [Contributing](#🙏-contributing)

## ✨ Features

- 🤖 **Unbeatable AI**: Uses minimax algorithm for optimal moves
- 🖥️ **Console Interface**: Simple and intuitive command-line interface
- ✅ **Input Validation**: Ensures valid moves only
- 🏆 **Win/Draw Detection**: Automatically detects game outcomes
- 🔄 **Replay Option**: Play again without restarting

## 🚀 Installation

1. 📥 **Install Rust**: Ensure you have [Rust installed](https://www.rust-lang.org/tools/install)
2. 📂 **Clone Repository**: `git clone https://github.com/your-repo/tictactoe.git`
3. 🛠️ **Build Project**: Run `cargo build` to compile
4. 🎮 **Start Game**: Run `cargo run` to begin
5. 🔧 **Development Mode**: Use `cargo watch -x run` for live reloading

## 🎯 How to Play

1. 🟦 **Game Board**: 3x3 grid with positions labeled 0-2
2. ❌ **Player**: You play as 'X'
3. ⭕ **Computer**: AI opponent plays as 'O'
4. ⌨️ **Input**: Enter row (0-2) then column (0-2) when prompted
5. 🏁 **Goal**: Get three of your marks in a row (horizontal, vertical, or diagonal)
6. 🤔 **Strategy**: Try to force a draw against the unbeatable AI

## 📝 Game Rules

1. 🔄 **Turns**: Players alternate placing 'X' or 'O'
2. 🏆 **Win**: First to get 3 in a row (any direction) wins
3. 🤝 **Draw**: Full board with no winner results in tie
4. ⛔ **Rules**:
   - Only empty spaces can be marked
   - Input must be valid coordinates (0-2)
   - No overwriting existing marks

## 🤖 About the AI

- 🧠 **Algorithm**: Minimax with alpha-beta pruning
- 🏆 **Performance**: Always makes optimal moves
- 🤝 **Outcome**: Unbeatable (but can be forced to draw)
- ⚡ **Speed**: Efficient decision making
- 🎲 **Strategy**: Considers all possible moves and outcomes

## 📜 License

📄 **MIT License** - Feel free to use, modify, and distribute!

```
Copyright (c) 2023 Your Name

Permission is hereby granted...
```

## 🙏 Contributing

🤝 **Contributions Welcome!**

1. 🐛 **Issues**: Report bugs or suggest features
2. 🛠️ **PRs**: Submit pull requests for improvements
3. 📝 **Guidelines**:
   - Follow Rust coding standards
   - Write clear commit messages
   - Include tests for new features
   - Update documentation accordingly

Enjoy the game! 🎉
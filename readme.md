# 🎯 Git Habit

> Because manually tracking your habits in a spreadsheet doesn't make you look cool...

---

Ever wanted to know exactly how much time you're spending on your side projects vs scrolling through social media? Well, this CLI tool won't stop you from doom-scrolling, but at least you'll have pretty graphs to show for it! 📊

## 📸 Photos:
![image](https://github.com/user-attachments/assets/1680ca54-c9ec-4aa1-a186-5ab54eb85a14)


## ✨ Features

- **Simple Logging**: Track your activities faster than you can say "I'll start being productive tomorrow"
  - The location depends on your operating system:
  - On Linux: `~/.local/share/habit-tracker/activities.json`
  - On macOS: `~/Library/Application Support/habit-tracker/activities.json`
  - On Windows: `C:\Users\[Username]\AppData\Roaming\habit-tracker\activities.json`
- **Pretty Visualizations**: Colorful terminal graphs that make your GitHub contribution graph jealous (Unless you're comparing tho the planet one)
- **Progress Tracking**: Watch your streak grow (or restart... we've all been there 😅)
- **Monthly Overview**: Because sometimes we need to prove to ourselves that we did *something* last month
- **Local Storage**: Your data stays on your machine (your questionable productivity habits are safe with us)

## 🚀 Installation

1. Make sure you have Rust installed (if not, [get it here](https://rustup.rs/))
2. Clone this repo:
```bash
git clone https://github.com/junque1r4/git-habit
cd git-habit
```
3. Build it:
```bash
cargo build --release
```
4. Create an alias by adding one of these to your shell's config file:
For Bash (`~/.bashrc`) or ZSH (`~/.zshrc`):
```bash
alias mycmd="$HOME/path/to/mycmd/target/release/mycmd"
```
For Fish (`~/.config/fish/config.fish`):
```bash
alias mycmd="$HOME/path/to/mycmd/target/release/mycmd"
```
For windows: (`why are you using Windows?`)
```bash
echo "Imagine the day i don't pretent Microsoft own's GitHub
```
5. Start tracking! (or procrastinate by reading the rest of this README)

## 📖 Usage

### Track an Activity
```bash
habit-tracker log <hours> "description"

# Examples:
habit-tracker log 1.5 "Actually wrote code instead of just thinking about it"
habit-tracker log 0.5 "Stared at Zed Editor trying to understand my own code"
```

### View Your Progress
```bash
habit-tracker view            # Shows last 365 days
habit-tracker view 30
```

## 🎨 Output

You'll get a beautiful dashboard showing:
- Current streak (aka "days since last excuse")
- Total hours (prepare to be either impressed or concerned)
- Daily averages (for when you need to feel productive)
- Pretty graphs (because we all love pretty graphs)

# 🔩 TODO:
Pray for me do it (Or create a PR and do it yourself, lazy. LOL)
- More log options (So you can lie about what you did another day. Or put something you forgot IDK)
- Create a more fancy way to install this (Or add to Crate? hmm)
- Create tests (Actually, test what?)

## 🤓 For Nerds (Technical Details)

Built with:
- Rust (because we're serious about performance... and looking cool)
- Chrono (time management for the program, not for you)
- Colored (making terminals pretty since whenever it was created)
- Serde (serialization that just works™)
- StructOpt (because typing is better than clicking... right?)
- A GalaxyBook3 360 (See? You don't need a MacBook M4 PRO to create simple stuff)

## 🤝 Contributing

Found a bug? Have a feature idea? Want to make the bar graphs even prettier? Contributions are welcome!

1. Fork it
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a PR
6. Wait while I pretend to thoroughly review your code

## 📝 License

MIT License or whatever IDC - Go wild! Just don't blame me if tracking your habits makes you realize how much time you spend "researching" on YouTube.

## 🙋‍♂️ FAQ

**Q: ☝️🤓Will this make me more productive?**
A: Technically, tracking your habits is already being productive! But results may vary...

**Q: ☝️🤓Can I track multiple habits?**
A: Yes! Though tracking "time spent tracking habits" might create an infinite loop.

**Q: ☝️🤓Is my data secure?**
A: It's stored locally on your machine. So unless your cat learns to read JSON, you're good.

---

Made with ❤️ and a lot of ☕️

*Remember: The first step to improving your habits is tracking them. The second step is actually doing them. This tool helps with the first part - you're on your own for the second!*

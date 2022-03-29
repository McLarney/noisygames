# Noisy Games 
#### Game Theory with Noisy Channels
---
The goal with this project is to study how games of strategy change as noise is introduced into the communications. Information asymmetry plays an important role in strategy and measurement. Sophisticated interaction with the world requires some carefully tuned sense of the quality of information delivered and received.

For starters, I will examine Iterated Prisoner's Dilemma. Firstly, simple strategies found from the Axelrod tournments will be employed. Eventually this work will build to include differential noise applied for "read" as well as "write" from the game.

## Base Game
[Iterative Prisoner's Dilemma](https://www.proseaic.com/iterated-prisoners-dilemma-with-rust/)

## Contemporary Research: Zero Determinant Strategies
Zero Determinant Strategies were found by William Press and Freeman Dyson. Upon figuring out that a longer memory does not help when playing against a player that does not use the opponent's score in their strategy, a series of manipulations can be found where one player's score can be directly tied to the actions of the other player entirely in terms of the strategy under a single player's control. A surprising outcome of this work is that it is quite possible to abide by this constraint and still extract an extortionate share.

The strategies employed here introduce Markov chains where the probabilities of the transitions between last round's moves played is the aggregate strategy. Because Markov chains can be solved for their steady state essentially as an eigenvector, the simulations performed are unnecessary but a useful setup for later work.

[Simulated Zero Determinant Strategies](https://www.proseaic.com/iterated-prisoners-dilemma-2-stochastic-zds/)

## Proposed Model
Still working on this.
### Read/Write in the Context of Games
We can view a player's interaction with a game with making moves is writing and reading the outcome as reading. The victory condition is still determined by what occurs in the actual game, but a player's strategy may be informed by incorrect information as to the outcome of the game.

### White Noise
White noise corresponds to a noise spectrum where the probability of an amount of noise is equally likely across a range. The noises of other colors correspond to different distributions from which the noise is drawn from. This impartial noise is fine as a baseline, although other distributions may be of interest. It may be expected though that more interesting information is to be found with types of noise that have directionality based on the signals considered.

### Markovian Noise
To treat this directionality, a Markovian model of noise will be used. This work will follow on from the work on white noise.

A motivating is the strategy of capturing a target dead or alive. A bounty asking for a target dead is unlikely to result in the target captured a live, but capturing a target alive may result in the target being killed. There is directionality between moves and their chance of becoming the other.

### Strategy Degredation
In an environment where there's uncertainty about uncertainties, it may be preferable to choose strategies that are robust over a wider range of noise. An initial approach would take strategies that perform well in the noiseless case, and then iterate over noise values, recording the output. It may be found that particular strategies are more resistant to noise than others.

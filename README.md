# aiball
8ball pool gym

## Todo

1. Be able to hit the cue ball with a desired force and cue angle.
2. Add interaction event manager.

## Pool Rules and Table Dimensions

- [Table Dimensions](https://www.dimensions.com/element/8-foot-billiards-pool-table)

- [Pocket Dimensions](https://www.dimensions.com/element/billiards-pool-table-pockets)

## Pool Agent

  - [Graph Representation Learning](https://www.cs.mcgill.ca/~wlh/grl_book/files/GRL_Book.pdf)
  - [Thing](https://arxiv.org/pdf/2004.13965.pdf)

## Physics
  - [Equations of Motion](https://ekiefl.github.io/2020/04/24/pooltool-theory/#--case-3-rolling)
  - [Algorithm](https://ekiefl.github.io/2020/12/20/pooltool-alg/)
  - [Blog](https://ekiefl.github.io/2020/04/24/pooltool-theory/)


## Notes

  - Causality cones
  - Priority queue for how soon something can happen to a given ball. Naturally dont get to that event until it becomes relevant.
  - Study time distributions D*(num_balls-2)/speed_sound


  "This means neither of the 2 balls engage in any transitions or collisions. If either did, then the intervening event necessarily precedes the i−j collision event, which means it doesn’t need to be considered as a candidate for the next event."
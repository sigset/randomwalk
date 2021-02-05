<h1 align="center">randomwalk</h1>

<img src="https://i.imgur.com/MxshZ1b.jpg" width=100%/>

<h5 align="center">
Build glorious random walks.
</h5>

<br/>

---

<br/>

```rust
use randomwalk::generators::NormalGenerator;

use randomwalk::translators::{
   UniformTranslator,
   ExponentialTranslator,
   LogNormalTranslator,
};
```

## Examples

### Helper methods used in the examples

```rust
// normal distribution between 0 and 1
let norm_dist = rand_distr::Normal::new(0.0, 1.0).unwrap();

// exponential distribution with lambda of 0.0055
let exp_dist = rand_distr::Exp::new(1.0 / 180.0).unwrap();

// uniform distribution between 0 and 1
let unif_dist = rand_distr::Uniform::new(0.0, 1.0);

let now = || chrono::Utc::now();

let mut rng = rand_hc::Hc128Rng::from_entropy();
```

### Normally distributed random walk with soft bound

This section demonstrates how to create a random walk that is soft bound to lie about its mean. To approximate a walk without a soft bound, give the variance a large value and use NormalGenerator.set to initialize the walk before calling NormalGenerator.next.

```rust
let utc_now = now();

# Setup a random walk with mean 100.0 and variance 250.0.
# The variance controls how tightly the walk is held
# close to its mean and sigma_xx controls how rapidly
# it can wander away from its starting point.

let mut normal_gen =
    NormalGenerator::new(
        100.0,
        250.0,
        0.1,
        utc_now.timestamp() as u64,
    );

let mut current_time = utc_now.timestamp_millis() as f64;
let mut current_value = normal_gen.next(current_time).unwrap();

for i in 1..10 {
    # pretend time lapsed
    current_time = current_time + (i as f64 * 500f64);
    current_value = normal_gen.next(current_time).unwrap();

    println!(
        "Normal walk value {} time {}",
        current_value,
        current_time,
    );
}
```

### Normally distributed random walk with soft bound and random shaped interpolation

This section demonstrates how to create a random walk that is soft bound to lie about the value of a shaping function.

```rust
let utc_now = now();

// Setup a random walk with zero mean. Since we'll be
// adding the value of the walk to the shaping functions,
// using a walk with zero mean ensures that the walk will
// wander around but will keep returning to the vicinity
// of the shaping function. Reducing the variance will
// keep the walk closer to the shaping function and
// make its form more obvious.

let mut normal_gen =
   NormalGenerator::new(
       100.0,
       250.0,
       0.1,
       utc_now.timestamp() as u64,
   );

// The walk will consist of shaped transitions between
// random points. The start variable represent the
// starting point of a transition, which is usually
// the 'current' time and value and the target variables
// represent the end point of a transition - the value
// the walk need to walk to and the time at which it
// needs to be there.

let mut current_time = utc_now.timestamp_millis() as f64;
let mut current_value = 0.0;

let mut start_time = current_time;
let mut start_value = 100.0 + rng.sample(norm_dist) * 10.0;

let mut target_time = current_time + rng.sample(exp_dist) + 5.0;
let mut target_value = 100.0 + rng.sample(norm_dist) * 10.0;

for i in 1..10 {
   current_time = current_time + (i as f64 * 500f64);

   if current_time > target_time {
       start_time = current_time;
       start_value = current_value;

       target_time = current_time + rng.sample(exp_dist) + 5.0;
       target_value = 100.0 + rng.sample(norm_dist) * 10.0;
   }

   current_value =
       normal_gen.next_interpolant(
           current_time,
           target_time,
           0.0,
       ).unwrap();

   let normalized_time =
       (current_time - start_time)
           / (target_time - start_time);

   // linear
   //let shaping_value =
   //    target_value
   //    + normalized_time
   //    + start_value * (1.0 - normalized_time);

   // smoothstep
   let shaping_value =
       start_value
           + (target_value - start_value)
           * (
               3.0 * normalized_time * normalized_time
               - 2.0 * normalized_time * normalized_time * normalized_time);

   // quarter
   //let shaping_value =
   //    start_value
   //    + (target_value - start_value)
   //        * (
   //            1.0 - (1.0 - normalized_time)
   //                * (1.0 - normalized_time)
   //        ).sqrt();

   //println!(
   //    "Shaping function {}",
   //    shaping_value,
   //);

   //println!(
   //    "Raw normal walk {}",
   //    current_value,
   //);

   println!(
       "Normal shaped interpolated walk {} @ {} target {} @ {}",
       current_value + shaping_value,
       current_time,
       target_value,
       target_time,
   );
}
```

### Normally distributed random walk with soft bound and random unshaped interpolation

This section demonstrates how to create a random walk that freely interpolates between fixed points without the help of a shaping function.

```rust
let utc_now = now();

// Setup a random walk with mean 100.0 and variance 250.0.
// The variance controls how tightly the walk is held
// close to its mean and sigma_xx controls how rapidly it
// can wander away from its starting point.

let mut normal_gen =
   NormalGenerator::new(
       100.0,
       250.0,
       0.1,
       utc_now.timestamp() as u64,
   );

// The walk will consist of unshaped transitions between
// random points. The start variable represent the
// starting point of a transition, which is usually
// the 'current' time and value and the target variables
// represent the end point of a transition - the value
// the walk need to walk to and the time at which it
// needs to be there.

let mut current_time = utc_now.timestamp_millis() as f64;
let mut current_value = normal_gen.next(current_time).unwrap();

let mut target_time = current_time + rng.sample(exp_dist) + 5.0;
let mut target_value = 100.0 + rng.sample(norm_dist) * 10.0;

for i in 1..10 {
   current_time = current_time + (i as f64 * 500f64);

   if current_time > target_time {
       target_time = current_time + rng.sample(exp_dist) + 5.0;
       target_value = 100.0 + rng.sample(norm_dist) * 10.0;
   }

   current_value =
       normal_gen.next_interpolant(
           current_time,
           target_time,
           target_value,
       ).unwrap();

   println!(
       "Normal unshaped interpolated walk {} @ {} target {} @ {}",
       current_value,
       current_time,
       target_value,
       target_time,
   );
}
```

### Log-Normally distributed random walk with soft bound

This section demonstrates how to create a log-normally distributed random walk. This is achieved by generating a normally distributed random walk and passing it through and log-normal translator that postprocesses it to have a log-normal distribution.

```rust
let utc_now = now();

// Generate a normally distributed random walk and attach
// it to a log-normal translator. Note that the
// log-normal translator raises the normal random walk
// to the power of e so we need to setup the mean
// and variance of the normal distribution so that
// the output of the log-normal translator has the
// properties that we want.

let mut normal_gen =
   NormalGenerator::new(
       100.0,
       250.0 / 100.0 / 100.0,
       0.1 / 100.0 / 100.0,
       utc_now.timestamp() as u64,
   );

let mut log_gen =
   LogNormalTranslator::new(
       normal_gen,
   );

let mut current_time = utc_now.timestamp_millis() as f64;
let mut current_value = log_gen.next(current_time).unwrap();

for i in 1..10 {
   current_time = current_time + (i as f64 * 500f64);

   current_value =
       log_gen.next(
           current_time,
       )
       .unwrap();

   println!(
       "Log-Normal walk {}",
       current_value,
   );
}
```

### Log-Normally distributed random walk with soft bound and random interpolations

This section demonstrates how to create a log-normally distributed random walk with unshaped random interpolations. For detailed comments please see the section for the log-normally distributed random walk and the normally distributed random walk with unshaped interpolations.

```rust
let utc_now = now();

let mut normal_gen =
   NormalGenerator::new(
       100.0,
       250.0 / 100.0 / 100.0,
       0.1 / 100.0 / 100.0,
       utc_now.timestamp() as u64,
   );

let mut log_gen =
   LogNormalTranslator::new(
       normal_gen,
   );

let mut current_time = utc_now.timestamp_millis() as f64;
let mut current_value = log_gen.next(current_time).unwrap();

for i in 1..10 {
   current_time = current_time + (i as f64 * 500f64);

   current_value =
       log_gen.next(
           current_time,
       )
           .unwrap();

   println!(
       "Log-Normal softbound walk {}",
       current_value,
   );
}
```

### Uniformly distributed random walk with soft bound

This section demonstrates how to create a uniformly distributed random walk. This is achieved by setting up a normally-distributed random walk and attaching it to a uniform translator, which converts it into a uniformly distributed random walk.

```rust
let utc_now = now();

// This section demonstrates how to create a uniformly
// distributed random walk. This is achieved by setting
// up a normally-distributed random walk and attaching
// it to a uniform translator, which converts it into
// a uniformly distributed random walk.

let mut normal_gen =
   NormalGenerator::new(
       100.0,
       250.0,
       0.0001,
       utc_now.timestamp() as u64,
   );

let mut uniform_tx =
   UniformTranslator::new(
       normal_gen,
   );

let mut current_time =
   utc_now.timestamp_millis() as f64;

let mut current_value =
   uniform_tx.next(
       current_time,
   )
   .unwrap();

for i in 1..10 {
   current_time = current_time + (i as f64 * 500f64);

   current_value =
       uniform_tx.next(
           current_time,
       )
       .unwrap();

   println!(
       "Uniform walk with softbound {} @ {}",
       current_value,
       current_time,
   );
}
```

### Uniformly distributed random walk with soft bound and random interpolation

This section demonstrates how to create a uniformly distributed random walk with unshaped random interpolations. For detailed comments please see the section for the uniformly distributed random walk and the normally distributed random walk with unshaped interpolations.

```rust
let utc_now = now();

let mut normal_gen =
   NormalGenerator::new(
       100.0,
       250.0,
       0.0001,
       utc_now.timestamp() as u64,
   );

let mut uniform_tx =
   UniformTranslator::new(
       normal_gen,
   );

let mut current_time =
   utc_now.timestamp_millis() as f64;

let mut current_value =
   uniform_tx.next(
       current_time,
   )
   .unwrap();

let mut target_time =
   current_time + rng.sample(exp_dist) + 5.0;

let mut target_value =
   rng.sample(unif_dist);

for i in 1..10 {
   current_time = current_time + (i as f64 * 500f64);

   if current_time > target_time {
       target_time =
           current_time + rng.sample(exp_dist) + 5.0;

       target_value =
           rng.sample(unif_dist);
   }

   current_value =
       uniform_tx.next_interpolant(
           current_time,
           target_time,
           target_value,
       )
       .unwrap();

   println!(
       "Uniform interpolated walk with softbound {} @ {} target {} @ {}",
       current_value,
       current_time,
       target_value,
       target_time,
   );
}
```

### Exponentially distributed random walk with soft bound

This section demonstrates how to create an expoentially distributed random walk. This is done by creating a normally distributed random walk and attaching it to a uniform translator to convert it to a uniformly distributed walk. The uniform translator is then attached to an exponential translator that converts the uniformly distributed walk into an exponentially distributed random walk. Note that the exponentially distributed random walk takes larger steps when it has a larger value.

```rust
let utc_now = now();
let time = utc_now.timestamp() as u64;

let normal_gen =
   NormalGenerator::new(
       100.0,
       250.0,
       0.000001,
       time,
   );

let uniform_tx =
   UniformTranslator::new(
       normal_gen,
   );

let mut exp_tx =
   ExponentialTranslator::new(
       100.0,
       uniform_tx,
   ).unwrap();

let mut current_time = utc_now.timestamp_millis() as f64;

let mut current_value =
   exp_tx.next(
       current_time,
   )
   .unwrap();

for i in 1..10 {
   current_time = current_time + (i as f64 * 500f64);

   current_value =
       exp_tx.next(
           current_time,
       )
       .unwrap();

   println!(
       "Exponential walk with softbound {} @ {}",
       current_value,
       current_time,
   );
}
```

### Exponentially distributed random walk with soft bound and random interpolations

This section demonstrates how to create a exponentially distributed random walk with unshaped random interpolations. For detailed comments please see the section for the exponentially distributed random walk and the normally distributed random walk with unshaped interpolations.

```rust
let utc_now = now();
let time = utc_now.timestamp() as u64;

let normal_gen =
   NormalGenerator::new(
       100.0,
       250.0,
       0.000001,
       time,
   );

let uniform_tx =
   UniformTranslator::new(
       normal_gen,
   );

let mut exp_tx =
   ExponentialTranslator::new(
       100.0,
       uniform_tx,
   ).unwrap();

let mut current_time = utc_now.timestamp_millis() as f64;

let mut current_value =
   exp_tx.next(
       current_time,
   )
       .unwrap();

let mut target_time =
   current_time
       + rng.sample(
       exp_dist,
   )
       + 5.0;

let mut target_value =
   100.0
       + rng.sample(
       norm_dist,
   )
       * 10.0;

for i in 1..10 {
   current_time = current_time + (i as f64 * 500f64);

   if current_time > target_time {
       target_time =
           current_time
               + rng.sample(
               exp_dist,
           )
               + 5.0;

       target_value =
           100.0
               + rng.sample(
               norm_dist,
           )
               * 10.0;
   }

   current_value =
       exp_tx.next_interpolant(
           current_time,
           target_time,
           target_value,
       )
           .unwrap();

   println!(
       "Exponential interpolated walk with softbound {} @ {} target {} @ {}",
       current_value,
       current_time,
       target_value,
       target_time,
   );
}
```

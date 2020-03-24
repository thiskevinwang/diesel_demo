# Rust Learnings

## Motivation

<small>(Why do I even want to learn Rust?)</small>

I want to pick up a low level language.
I don't want to learn C-anything.
There a some pieces of tech, built on Rust that I'd like to understand.

- [Deno](https://deno.land/)
- [AWS Firecracker](https://aws.amazon.com/blogs/aws/firecracker-lightweight-virtualization-for-serverless-computing/)

## Topics

- [Diesel](http://diesel.rs/) (ORM)
- [Actix](https://actix.rs/) (Actor System & Web Framework)
- Lambda
  - https://github.com/awslabs/aws-lambda-rust-runtime
  - https://github.com/softprops/lambda-rust

### Random Findings

<details open>
<summary>Closure Syntax</summary>

```ts
// TypeScript
function main(): void {
  const foo = "weird";

  // ES6 Arrow Function
  ((val: string) => {
    console.log(`${val}, self-calling arrow function`);
  })(foo);
}
// weird, self-calling arrow function
```

```rust
// Rust
fn main() -> () {
    let foo = "weird";

    // Rust Closure/Lambda
    (|val: &str| {
        println!("{}, self-calling closure/lambda", val);
    })(foo);
}
// weird, self-calling closure/lambda
```

```rust
// Rust
// (not self-calling)
use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let duration = start.elapsed();

    let myClosure = ||println!("Time elapsed: {:?}", duration);
    myClosure();
}
// Time elapsed: 925ns
```

</details>

## Resources

https://www.heroku.com/podcasts/codeish/34-an-introduction-to-rust

## Outstanding Questions

- [ ] How do you deploy a Rust application?
- [ ] How do you deploy a Rust lambda function?

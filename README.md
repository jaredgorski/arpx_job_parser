<div>
  <h1 align="left">
    arpx_job_parser
  </h1>
  <p align="left">Parse arpx job scripts for runtime</p>
</div>

<div align="center">
  <h2>
    Description
  </h2>
</div>

This library provides parser functions for processing arpx job scripts.

<div align="center">
  <h2>
    Usage
  </h2>
</div>

```rust
use arpx_job_parser::parse_job;

fn main() {
    let job = r#"
        [
            (loop1 ? loop2 : loop3;)
            loop2 ? loop3 : loop4;
        ]
        loop3 ? loop4 : loop5;
        loop6;
        (loop7 ? loop8;)
    "#;

    let parsed = parse_job(job);

    dbg!(parsed);
}

// [src/main.rs:16] parsed = Ok(
//     (
//         "",
//         Job {
//             tasks: [
//                 Task {
//                     processes: [
//                         Process {
//                             name: "loop1",
//                             onfail: "loop3",
//                             onsucceed: "loop2",
//                             silent: true,
//                         },
//                         Process {
//                             name: "loop2",
//                             onfail: "loop4",
//                             onsucceed: "loop3",
//                             silent: false,
//                         },
//                     ],
//                 },
//                 Task {
//                     processes: [
//                         Process {
//                             name: "loop3",
//                             onfail: "loop5",
//                             onsucceed: "loop4",
//                             silent: false,
//                         },
//                     ],
//                 },
//                 Task {
//                     processes: [
//                         Process {
//                             name: "loop6",
//                             onfail: "",
//                             onsucceed: "",
//                             silent: false,
//                         },
//                     ],
//                 },
//                 Task {
//                     processes: [
//                         Process {
//                             name: "loop7",
//                             onfail: "",
//                             onsucceed: "loop8",
//                             silent: true,
//                         },
//                     ],
//                 },
//             ],
//         },
//     ),
// )
```

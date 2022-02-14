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
use arpx_job_parser::job;

let job = r#"
    [
        (loop1 ? loop2 : loop3;)
        loop2 ? loop3 : loop4;
    ]
    loop3 ? loop4 : loop5;
    loop6;
    (loop7 ? loop8;)
"#;

let parsed = job().parse(example);

dbg!(parsed);

// Ok((
//     "",
//     Job {
//         tasks: vec![
//             Task {
//                 processes: vec![
//                     Process {
//                         name: "loop1".to_string(),
//                         onsucceed: "loop2".to_string(),
//                         onfail: "loop3".to_string(),
//                         silent: true,
//                     },
//                     Process {
//                         name: "loop2".to_string(),
//                         onsucceed: "loop3".to_string(),
//                         onfail: "loop4".to_string(),
//                         silent: false,
//                     },
//                 ],
//             },
//             Task {
//                 processes: vec![Process {
//                     name: "loop3".to_string(),
//                     onsucceed: "loop4".to_string(),
//                     onfail: "loop5".to_string(),
//                     silent: false,
//                 }],
//             },
//             Task {
//                 processes: vec![Process {
//                     name: "loop6".to_string(),
//                     onsucceed: String::new(),
//                     onfail: String::new(),
//                     silent: false,
//                 }],
//             },
//             Task {
//                 processes: vec![Process {
//                     name: "loop7".to_string(),
//                     onsucceed: "loop8".to_string(),
//                     onfail: String::new(),
//                     silent: true,
//                 }],
//             },
//         ],
//     },
// ))
```

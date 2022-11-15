# Checklist for conformance to Rust API guidelines

copied from [Rust API Guidelines - Checkliste](https://github.com/rust-lang/api-guidelines/blob/master/src/checklist.md), dated Aug 23, 2021, commit: [b2f62d6](https://github.com/rust-lang/api-guidelines/blob/master/src/checklist.md).

&#x2714;: done | &#x1F4C5; (x.y): planned in release x.y | &#x26AA;: n/a | &#x274C;: failed

## Naming

*(crate aligns with Rust naming conventions)* <br/>

  &#x2714; Casing conforms to RFC 430 ([C-CASE]) <br/>
  &#x26AA; Ad-hoc conversions follow `as_`, `to_`, `into_` conventions ([C-CONV]) <br/>
  &#x2714; Getter names follow Rust convention ([C-GETTER]) <br/>
  &#x1F4C5; (0.6) Methods on collections that produce iterators follow `iter`, `iter_mut`, `into_iter` ([C-ITER]) <br/>
  &#x1F4C5; (0.6) Iterator type names match the methods that produce them ([C-ITER-TY]) <br/>
  &#x26AA; Feature names are free of placeholder words ([C-FEATURE]) <br/>
  &#x274C; Names use a consistent word order ([C-WORD-ORDER]) <br/>

## Interoperability

*(crate interacts nicely with other library functionality)* <br/>

  &#x2714; Types eagerly implement common traits *(`Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `Default`)* ([C-COMMON-TRAITS]) <br/>
  &#x26AA; Conversions use the standard traits `From`, `AsRef`, `AsMut` ([C-CONV-TRAITS]) <br/>
  &#x1F4C5; (0.6) Collections implement `FromIterator` and `Extend` ([C-COLLECT]) <br/>
  &#x1F4C5; (0.5) Data structures implement Serde's `Serialize`, `Deserialize` ([C-SERDE]) <br/>
  &#x26AA; Types are `Send` and `Sync` where possible ([C-SEND-SYNC]) <br/>
  &#x1F4C5; (0.3) Error types are meaningful and well-behaved ([C-GOOD-ERR]) <br/>
  &#x26AA; Binary number types provide `Hex`, `Octal`, `Binary` formatting ([C-NUM-FMT]) <br/>
  &#x26AA; Generic reader/writer functions take `R: Read` and `W: Write` by value ([C-RW-VALUE]) <br/>

## Macros

*(crate presents well-behaved macros)* <br/>

  &#x26AA; Input syntax is evocative of the output ([C-EVOCATIVE]) <br/>
  &#x26AA; Macros compose well with attributes ([C-MACRO-ATTR]) <br/>
  &#x26AA; Item macros work anywhere that items are allowed ([C-ANYWHERE]) <br/>
  &#x26AA; Item macros support visibility specifiers ([C-MACRO-VIS]) <br/>
  &#x26AA; Type fragments are flexible ([C-MACRO-TY]) <br/>

## Documentation

*(crate is abundantly documented)* <br/>

  &#x2714; Crate level docs are thorough and include examples ([C-CRATE-DOC]) <br/>
  &#x1F4C5; (0.3) All items have a rustdoc example ([C-EXAMPLE]) <br/>
  &#x2714; Examples use `?`, not `try!`, not `unwrap` ([C-QUESTION-MARK]) <br/>
  &#x2714; Function docs include error, panic, and safety considerations ([C-FAILURE]) <br/>
  &#x1F4C5; (0.3) Prose contains hyperlinks to relevant things ([C-LINK]) <br/>
  &#x2714; Cargo.toml includes all common metadata *(authors, description, license, homepage, documentation, repository, keywords, categories)* ([C-METADATA]) <br/>
  &#x2714; Release notes document all significant changes ([C-RELNOTES]) <br/>
  &#x2714; Rustdoc does not show unhelpful implementation details ([C-HIDDEN]) <br/>

## Predictability

*(crate enables legible code that acts how it looks)* <br/>

  &#x26AA; Smart pointers do not add inherent methods ([C-SMART-PTR]) <br/>
  &#x26AA; Conversions live on the most specific type involved ([C-CONV-SPECIFIC]) <br/>
  &#x2714; Functions with a clear receiver are methods ([C-METHOD]) <br/>
  &#x2714; Functions do not take out-parameters ([C-NO-OUT]) <br/>
  &#x26AA; Operator overloads are unsurprising ([C-OVERLOAD]) <br/>
  &#x26AA; Only smart pointers implement `Deref` and `DerefMut` ([C-DEREF]) <br/>
  &#x2714; Constructors are static, inherent methods ([C-CTOR]) <br/>

## Flexibility

*(crate supports diverse real-world use cases)* <br/>

  &#x2714; Functions expose intermediate results to avoid duplicate work ([C-INTERMEDIATE]) <br/>
  &#x1F4C5; (0.3) Caller decides where to copy and place data ([C-CALLER-CONTROL]) <br/>
  &#x2714; Functions minimize assumptions about parameters by using generics ([C-GENERIC]) <br/>
  &#x2714; Traits are object-safe if they may be useful as a trait object ([C-OBJECT]) <br/>

## Type safety

*(crate leverages the type system effectively)* <br/>

  &#x26AA; Newtypes provide static distinctions ([C-NEWTYPE]) <br/>
  &#x1F4C5; (0.3) Arguments convey meaning through types, not `bool` or `Option` ([C-CUSTOM-TYPE]) <br/>
  &#x26AA; Types for a set of flags are `bitflags`, not enums ([C-BITFLAG]) <br/>
  &#x26AA; Builders enable construction of complex values ([C-BUILDER]) <br/>

## Dependability

*(crate is unlikely to do the wrong thing)* <br/>

  &#x2714; Functions validate their arguments ([C-VALIDATE]) <br/>
  &#x26AA; Destructors never fail ([C-DTOR-FAIL]) <br/>
  &#x26AA; Destructors that may block have alternatives ([C-DTOR-BLOCK]) <br/>

## Debuggability

*(crate is conducive to easy debugging)* <br/>

  &#x2714; All public types implement `Debug` ([C-DEBUG]) <br/>
  &#x2714; `Debug` representation is never empty ([C-DEBUG-NONEMPTY]) <br/>

## Future proofing

*(crate is free to improve without breaking users' code)* <br/>

  &#x26AA; Sealed traits protect against downstream implementations ([C-SEALED]) <br/>
  &#x2714; Structs have private fields ([C-STRUCT-PRIVATE]) <br/>
  &#x26AA; Newtypes encapsulate implementation details ([C-NEWTYPE-HIDE]) <br/>
  &#x2714; Data structures do not duplicate derived trait bounds ([C-STRUCT-BOUNDS]) <br/>

## Necessities

*(to whom they matter, they really matter)* <br/>

  &#x2714; Public dependencies of a stable crate are stable ([C-STABLE]) <br/>
  &#x2714; Crate and its dependencies have a permissive license ([C-PERMISSIVE]) <br/>


[C-CASE]: https://rust-lang.github.io/api-guidelines/naming.html#c-case
[C-CONV]: https://rust-lang.github.io/api-guidelines/naming.html#c-conv
[C-GETTER]: https://rust-lang.github.io/api-guidelines/naming.html#c-getter
[C-ITER]: https://rust-lang.github.io/api-guidelines/naming.html#c-iter
[C-ITER-TY]: https://rust-lang.github.io/api-guidelines/naming.html#c-iter-ty
[C-FEATURE]: https://rust-lang.github.io/api-guidelines/naming.html#c-feature
[C-WORD-ORDER]: https://rust-lang.github.io/api-guidelines/naming.html#c-word-order

[C-COMMON-TRAITS]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits
[C-CONV-TRAITS]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-conv-traits
[C-COLLECT]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-collect
[C-SERDE]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-serde
[C-SEND-SYNC]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-send-sync
[C-GOOD-ERR]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err
[C-NUM-FMT]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-num-fmt
[C-RW-VALUE]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-rw-value

[C-EVOCATIVE]: https://rust-lang.github.io/api-guidelines/macros.html#c-evocative
[C-MACRO-ATTR]: https://rust-lang.github.io/api-guidelines/macros.html#c-macro-attr
[C-ANYWHERE]: https://rust-lang.github.io/api-guidelines/macros.html#c-anywhere
[C-MACRO-VIS]: https://rust-lang.github.io/api-guidelines/macros.html#c-macro-vis
[C-MACRO-TY]: https://rust-lang.github.io/api-guidelines/macros.html#c-macro-ty

[C-CRATE-DOC]: https://rust-lang.github.io/api-guidelines/documentation.html#c-crate-doc
[C-EXAMPLE]: https://rust-lang.github.io/api-guidelines/documentation.html#c-example
[C-QUESTION-MARK]: https://rust-lang.github.io/api-guidelines/documentation.html#c-question-mark
[C-FAILURE]: https://rust-lang.github.io/api-guidelines/documentation.html#c-failure
[C-LINK]: https://rust-lang.github.io/api-guidelines/documentation.html#c-link
[C-METADATA]: https://rust-lang.github.io/api-guidelines/documentation.html#c-metadata
[C-HTML-ROOT]: https://rust-lang.github.io/api-guidelines/documentation.html#c-html-root
[C-RELNOTES]: https://rust-lang.github.io/api-guidelines/documentation.html#c-relnotes
[C-HIDDEN]: https://rust-lang.github.io/api-guidelines/documentation.html#c-hidden

[C-SMART-PTR]: https://rust-lang.github.io/api-guidelines/predictability.html#c-smart-ptr
[C-CONV-SPECIFIC]: https://rust-lang.github.io/api-guidelines/predictability.html#c-conv-specific
[C-METHOD]: https://rust-lang.github.io/api-guidelines/predictability.html#c-method
[C-NO-OUT]: https://rust-lang.github.io/api-guidelines/predictability.html#c-no-out
[C-OVERLOAD]: https://rust-lang.github.io/api-guidelines/predictability.html#c-overload
[C-DEREF]: https://rust-lang.github.io/api-guidelines/predictability.html#c-deref
[C-CTOR]: https://rust-lang.github.io/api-guidelines/predictability.html#c-ctor

[C-INTERMEDIATE]: https://rust-lang.github.io/api-guidelines/flexibility.html#c-intermediate
[C-CALLER-CONTROL]: https://rust-lang.github.io/api-guidelines/flexibility.html#c-caller-control
[C-GENERIC]: https://rust-lang.github.io/api-guidelines/flexibility.html#c-generic
[C-OBJECT]: https://rust-lang.github.io/api-guidelines/flexibility.html#c-object

[C-NEWTYPE]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-newtype
[C-CUSTOM-TYPE]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-custom-type
[C-BITFLAG]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-bitflag
[C-BUILDER]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder

[C-VALIDATE]: https://rust-lang.github.io/api-guidelines/dependability.html#c-validate
[C-DTOR-FAIL]: https://rust-lang.github.io/api-guidelines/dependability.html#c-dtor-fail
[C-DTOR-BLOCK]: https://rust-lang.github.io/api-guidelines/dependability.html#c-dtor-block

[C-DEBUG]: https://rust-lang.github.io/api-guidelines/debuggability.html#c-debug
[C-DEBUG-NONEMPTY]: https://rust-lang.github.io/api-guidelines/debuggability.html#c-debug-nonempty

[C-SEALED]: https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed
[C-STRUCT-PRIVATE]: https://rust-lang.github.io/api-guidelines/future-proofing.html#c-struct-private
[C-NEWTYPE-HIDE]: https://rust-lang.github.io/api-guidelines/future-proofing.html#c-newtype-hide
[C-STRUCT-BOUNDS]: https://rust-lang.github.io/api-guidelines/future-proofing.html#c-struct-bounds

[C-STABLE]: https://rust-lang.github.io/api-guidelines/necessities.html#c-stable
[C-PERMISSIVE]: https://rust-lang.github.io/api-guidelines/necessities.html#c-permissive

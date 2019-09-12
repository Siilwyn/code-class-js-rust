    What this code class is about
        This talk will be a dive into what Rust is about
        Approaching it from what I know from the JS and Node.js world
        A bit on how to use it
        Won't go in depth on language details: Rust book & learn at your own pace
    Introduction
        Rust is a programming language
        Goal: A language empowering everyone to build reliable and efficient software
        A young modern language, learning from past mistakes
        Big picture: Rust works for low-level system programming but also to create apps and website
    How?
        Statically typed
        types are inferred where possible
        Compiled
        strict & friendly compiler
        Rust forces you to think upfront about
            code structure
            API design
            edge cases
        Because
            No garbage collection
            Lots of (enforced) conventions
            Compiler is strict, e.g. string handling
        Makes it less suited for rapid prototyping
        I found more dependencies to use native system libraries which sucks
    Why?
        In general, learning a language provides more programming insight, useful for my JS too
        Rust is the most loved programming language on StackOverflow for 4 years in a row
        Offers complete package, think less about non-code
        The Rust standard distribution includes tools to run code, manage docs and packages
        Compare to JS
            How do you start a project?
                npm as standard, cargo: what comes by default
                Cargo creates a `src/main.rs`
            testing: unit tests and integration tests baked in
            std: contains a lot more methods for nicely handling things like padding strings and can grow because less used scopes are 'imported' just like Node.js and unlike the web
            Async stuff is not nice when you need to deal with it
    Who?
        Mozilla
            Developed internally for Firefox
            Rust is in development for 9 years
            4 years ago version 1.0 got released
            Put in the open as open source organization
        Used in the back-end of npm, Figma and Dropbox
            https://www.rust-lang.org/static/pdfs/Rust-npm-Whitepaper.pdf
            https://www.figma.com/blog/rust-in-production-at-figma/
            https://blogs.dropbox.com/tech/2016/06/lossless-compression-with-brotli/
        "Rust has absolutely stunning dependency management." - engineer at npm
        Used as webassembly source at Cloudflare
        https://developers.cloudflare.com/workers/webassembly/
    Let's try it out
        Go through hello world example
            cd /tmp
            nvim hello.rs
            rustup doc > open standard library
            search for log > print
            println!("Hello world!")
            rustc hello.rs
            rustc --explain E0601
            ./hello
        Setup
            Rustup (version manager)
            Don't worry removing is simple
        Exercises
        use cargo test to show how are unit and integrations tests done, how is documentation generated
            Exercise 1
                Introduction
                    two functions, have to change things in both
                    on line 8 we have to add a value, hint: check the other print statement
                variables immutable by default
                naming conventions
                functions need type annotation, rest inferred where possible
            Exercise 2
            let's talk to the web!
                Introduction
                    importing the rust standard library and external dependencies: crates
                    define structure, hashmap and vector: types, object and array
                    see links to the docs of what 'iter' and 'get' return
                    note that there are ways to write more elegant, but kept it more explicit for the code class
                cargo 'syncs' dependencies automatically
                need to handle errors, no null
                    Option.unwrap is explicit easy way out
                    Option has methods to operate on internal values
                JSON to typed responses
    Conclusion
        Still a bit young, development tools are catching up
        Async support in nightly but not stable yet
        Rust for node developers has a great introduction but the last few chapters are a bit out of date

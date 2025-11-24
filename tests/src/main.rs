fn main() {}

#[cfg(test)]
mod tests {
    use fast_struct::*;

    #[test]
    fn auto_getters_test() {
        #[derive(AutoGetters)]
        struct Foo {
            bar: String,
            baz: i16,
        }

        let foo = Foo {
            bar: "Hello, world!".into(),
            baz: 1984,
        };

        assert_eq!("Hello, world!", foo.get_bar());
        assert_eq!(1984_i16, *foo.get_baz());
    }
    
    #[test]
    fn auto_setters_test() {
        #[derive(AutoSetters)]
        struct Foo {
            bar: String,
            baz: i16,
        }
        
        let mut foo = Foo {
            bar: "Hi, world!".to_string(),
            baz: 2020,
        };
        
        foo.set_bar("Hello, world!");
        foo.set_baz(1984_i16);
    }

    #[test]
    fn optional_test() {
        #[optional]
        struct Foo {
            bar: String,
            baz: usize,
        }

        let foo = Foo {
            bar: Some("Hello, world!".to_string()),
            baz: Some(1984),
        };

        assert_eq!(Some("Hello, world!".to_string()), foo.bar);
        assert_eq!(Some(1984), foo.baz);
    }

    #[test]
    fn builder_test() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Builder)]
        struct Foo {
            bar: String,
            baz: i16,
            qux: bool,
        }

        let foo = Foo::builder()
            .bar("Hello, world!")
            .baz(1984_i16)
            .qux(true)
            .build()?;

        assert_eq!("Hello, world!", foo.bar);
        assert_eq!(1984_i16, foo.baz);
        assert_eq!(true, foo.qux);

        Ok(())
    }
}

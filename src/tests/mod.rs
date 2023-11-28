pub mod tests {
    use crate::Interpreter;

    #[test]
    pub fn hello_world() {
        let mut i: Interpreter = Interpreter::new();
        i.parse(String::from("
        ++++++++++++[>++++++<-]>.           H
        >++++++++++[>++++++++++<-]>+.       e
        >+++++++++[>++++++++++++<-]>..      l (printed twice)
        >++++++++++[>+++++++++++<-]>+.      o
        >++++[>+++++++++++<-]>.             (comma)
        >++++[>++++++++<-]>.                (space)
        >++++++++[>+++++++++++<-]>-.        W
        >++++++++++[>+++++++++++<-]>+.      o
        >++++++++++[>+++++++++++<-]>++++.   r
        >+++++++++[>++++++++++++<-]>.       l
        >++++++++++[>++++++++++<-]>.        d
        >++++[>++++++++<-]>+.               (exclamation)
        >+++[>+++<-]>+.                     (new line)
    ")).expect("Hello world test failed");
    }
}
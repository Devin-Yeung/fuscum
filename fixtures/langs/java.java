// This is a Java comment
public class HelloWorld {
    private String message;
    
    public HelloWorld(String msg) {
        this.message = msg;
    }
    
    public void greet(String name) {
        String greeting = "Hello, " + name;
        System.out.println(greeting);
    }
    
    public static void main(String[] args) {
        HelloWorld hello = new HelloWorld("Welcome");
        hello.greet("World");
    }
}

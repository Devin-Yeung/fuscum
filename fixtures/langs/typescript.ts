// This is a TypeScript comment
interface User {
    name: string;
    age: number;
}

function greet(user: User): string {
    const message = "Hello, " + user.name;
    return message;
}

const currentUser: User = { name: "Alice", age: 30 };
console.log(greet(currentUser));

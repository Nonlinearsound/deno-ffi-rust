const library = Deno.dlopen("./target/debug/my_library.dll",{
    add: {
        parameters: ["f64","f64"],
        result: "f64",
    } ,
    print_string: {
        parameters: ["buffer"],
        result: "void"
    },
    get_mouse_pos: {
        parameters: ["buffer"],
        result: "bool"
    }
});

const results = library.symbols.add(10,45.5);
console.log(results);

const my_string = "Hello world. From Javascript\0";
const str_pointer = new TextEncoder().encode(my_string);
library.symbols.print_string(str_pointer);

interface Coordinate {
    x: number,
    y: number
}

export function getCursorPos(): Coordinate {
    let coordinates = new Int32Array(2);
    if (library.symbols.get_mouse_pos(coordinates) == false) {
        throw "Could not get cursor position";
    }
    return {x:coordinates[0],y:coordinates[1]};
}

while (true) {
    console.log(getCursorPos());
}

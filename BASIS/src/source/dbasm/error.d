module dbasm.error;
import std.string;
import std.stdio;

struct Error
{
    string message;
    size_t line;

    void print()
    {
        "Error message: %s".format(message).writeln;
        "At line: %d".format(line).writeln;
    }
}

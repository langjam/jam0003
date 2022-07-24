module dbasm.operations;
import dbasm;

struct Operation
{
    tokens operator;
    string[] registers; // first register is the source register
    AssemblyValue[] values;
}

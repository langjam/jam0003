module dbasm.box;
import std.string;
import std.conv;
import box;


class AssemblyValue : Box!ulong
{
    private auto _isMemoryLocation = false;

    auto isMemoryLocation()
    {
        return _isMemoryLocation;
    }

    this(string rawValue)
    {
        super(0);

        if (rawValue.startsWith("#"))
        {
            _isMemoryLocation = true;
            rawValue = rawValue[1..rawValue.length];
        }

        value = rawValue.to!ulong;
    }
}

module box;
import std.string;

abstract class BoxedValue
{
    TypeInfo getType();
}

class Box(T) : BoxedValue
{
    T value;
    alias value this;

    this(T t)
    {
        value = t;
    }

    override TypeInfo getType()
    {
        return typeid(value);
    }

    override string toString()
    {
        return "%s".format(value);
    }
}

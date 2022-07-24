# Beautiful Cheese
We took the "Beautiful" from the theme "Beautiful assembly" and decided to simplify and beautify ruby's syntax

## features

new function syntax inspired by rust
```ruby
double = |n| n * 2

print_twice = |str|(
    puts str
    puts str
)

random_number = || 4
```
new way to use functions instead of blocks
```rb
[1,2,3].map(&double)

[1,2,3].map(|x| x * 2)
```
new way to use method calls
```rb
hex = .to_s(16)
[10, 20, 30].map(&hex)

["10", "20", "30"].map(.to_i)
``` 

# running
you need ruby 2.6 or later <br>
then run
```sh
ruby main.rb sourcefile outfile
```
the transpiled ruby code will be put in outfile and automatically executed

# more features
most things doesn't work...<br>
control flow with ternary / short circuit<br>
strings<br>
numbers<br>
arrays<br>
variables<br>
math<br>
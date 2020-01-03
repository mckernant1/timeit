# timeit
A linux timer that times the following commands.

It will print the time in milliseconds after the process exits 

## To Install 
```bash
brew tap mckernant1/tools
brew install timeit
```

### Available Options

--disable-output (-d)

Disables the output of the command

--times (-t) <INT>

Runs the command the command the specified amount of times. 
Prints all results in order at the end of any output


## Example usage
```bash
timeit npm run build
```

```bash
timeit brew update
```

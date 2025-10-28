<p align="center">
    <picture>
      <img src="./img.png">
    </picture>
</p>
<p align="center">Helper for searching individual colors in your terminal</p>
<p align="center">
</p>

### Idea

The idea came after visiting the site with palettes and I thought there were so many uniform and non-tacky color palettes, that I decided to create a tool that will give much more individual colors based on computer random.

### Installation
#### Check the realese page

or

#### Build from source code

Install the necessary dependencies
```bash
 sudo apt install libcairo2-dev libfreetype-dev clang curl unzip
```
Compile binary
```bash
bun run src/init.ts && bun build ./src/index.tsx --compile --dynamic --outfile color-hunter 
```

### Contributing

Welcome your any comments about my code and also help.
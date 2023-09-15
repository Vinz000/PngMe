# PNGme
## About
A cli tool to encode messages into PNG files using [Picklenerd's guide](https://picklenerd.github.io/pngme_book/introduction.html).

## Installation
Method 1: Clone the repo and install using cargo.
```
$ git clone https://github.com/Vinz000/pngme.git
$ cd ./pngme
$ cargo install --path .
```

Method 2: Install directly from git
```
$ cargo install --git https://github.com/Vinz000/pngme.git
```

## Usage
**Encode**
```
$ pngme encode <input> <chunk_type> <message> [output]
```

**Decode**
```
$ pngme decode <input> <chunk_type>
```

**Remove**
```
$ pngme remove <input> <chunk_type>
```

**Print**
```
$ pngme print <input>
```

## Example
This example will encode `test.png` using chunk type `RuSt`

**Encode**
```
$ pngme encode test.png RuSt "Message to encode" res.png
```

**Decode**
```
$ pngme decode res.png RuSt
```

**Remove**
```
$ pngme remove res.png RuSt
```

**Print**
```
$ pngme print res.png
```

# cons-wang06 
A Rust program that calculates conservation score a site in Multiple Sequence Alignment (MSA) by Relative Entropy. 

## Description 
* This program calculates conservation score in each site of MSA. 
* The scoring measure is Relative Entropy (RE) applied by Wang and Samudrala [1]. 
* It takes account of background distribution, sequence weighting and gap penalty. 

## Dependencies 

* `colored` ( https://github.com/mackwic/colored ) 

``` 
[dependencies]
colored = "2.0"
``` 

## Installation 

You can compile this program by using `Cargo`. 📦🦀

[e. g.] 

``` 
% cd cons-re-main
% cargo build --release
``` 
Then the object file will be generated in `./target/release` directory. 

## Scoring method 

### Conservation score 
The conservation score is calculated by using RE as follows: 

<img width="1440" alt="image01_cons-re" src="https://user-images.githubusercontent.com/83740080/137861859-014ae727-6ed0-46f4-8a38-624deca3e9d6.png">

where ***p*** is the amino acid probability of site ***C*** and ***q*** is a previously given background distribution. ⚠️NOTE that gaps are ignored when it constructs a site distribution. 

### Sequence weighting 
This program supports two types of sequence weighting: 

1. The Position-Based method by Henikoff and Henikoff [2].
2. The Distance-Based method by Vingron and Argos [3]. 

### Gap penalty 
The gap penalties are given as follows:

<img width="1440" alt="image02_cons-re" src="https://user-images.githubusercontent.com/83740080/137862202-11f40bdb-8650-4c19-b8ba-172faaad7393.png">

where ***L*** is the length of a site (or number of the sequences in MSA). The gap penalty of site ***i*** is given by calculating sum of weighting factors assigned to the gaps in sequence ***j***. 

### Back ground distribution 
RE requires a background distribution. In this program, there are three nine of background distributions you can choose: 

1. BLOSUM62 
2. AA composition of Swiss-Prot (from UniProt Knowledge Base) 
3. AA composition in extracellular proteins 
4. AA composition in membrane proteins
5. AA composition in intracellular proteins
6. JTT
7. WAG
8. LG
9. Non-biassed distribution (equal rate at 0.05)  

Gaps are ignored as well as site distributions.  

## Input file format
Aligned Multi-FASTA format of amino acid sequences. ⚠️ NOTE that nucleotide sequences are not supported.

See some example input files in `demo` directory.

## Usage
Major arguments:

`-i` : Input filename in aligned Multi-FASTA format, REQUIRED.

`-o` : Output filename, REQUIRED.

`-w` : Method of sequence weighting ( "hen" or "va", default "hen" ).

`-b` : Background distribution (default "blosum62").

[e. g.]

```
% ./cons-re -i input.fasta -o output.txt -w va -c yes -t no
```
Type `-h` to see other available options.

## Output

Number`\t`Conservation score`\t`Composition of the site

[e.g.]

<img width="1016" alt="image03_cons-re" src="https://user-images.githubusercontent.com/83740080/137862540-88840b76-e983-44bf-bcc3-e7ef09abec50.png">

## References 
1. Wang, Kai, and Ram Samudrala. "Incorporating background frequency improves entropy-based residue conservation measures." BMC bioinformatics 7.1 (2006): 1-8.
2. Henikoff, Steven, and Jorja G. Henikoff. "Position-based sequence weights." Journal of molecular biology 243.4 (1994): 574-578. 
3. Vingron, Martin, and Patrick Argos. "A fast and sensitive multiple sequence alignment algorithm." Bioinformatics 5.2 (1989): 115-121.

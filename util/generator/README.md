# Report Generator

Print on stdout a report for GCCRS

## Usage

### With the shell script

./weekly.sh \<path/to/repo\>: make a report for the last week, first argument is a path to the local repository of gccrs

./monthly.sh \<path/to/repo\>: make a report for the last month, first argument is a path to the local repository of gccrs

### With cargo

cargo r -- --kind \<KIND\> --date \<DATE\> --author \<AUTHOR\> --token \<TOKEN\>

> KIND: "monthly"/"weekly"
>
> DATE: YYYY-MM-DD (you can use `date -I` to get the good format)
>
>  AUTHOR: String specifying the author
>  
>  TOKEN: GitHub token used to access the repository

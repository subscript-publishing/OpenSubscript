# Command Syntax
## Attributes
```
[key]
[key, …]
[key=value]
[key=value, …]
[key key=value …]
```

## Labeled (Trailing) Enclosures
```
@name {…}
@name […]
@name <name>…</name>
@kind type {…}
@kind type […]
@kind type <name>…</name>
```

## Backslash Command Syntax
```
\cmd
\cmd[…]{…}
\cmd[…]{…} @label {…}
\cmd[…]{…} @label-kind label-type {…}
\cmd[…]{…} @label []
\cmd[…]{…} @label-kind label-type []
```

### With Labeled Enclosure/Argument
```
\cmd
\cmd[…]{…}
\cmd[…]{…} @label {…}
\cmd[…]{…} @label-kind label-type {…}
\cmd[…]{…} @label []
\cmd[…]{…} @label-kind label-type []
```

## Pipe Command Syntax
```
|cmd
|cmd[…]{…}
|cmd[…]{…} @label {…}
|cmd[…]{…} @label-kind label-type {…}
|cmd[…]{…} @label []
|cmd[…]{…} @label-kind label-type []    
```

## Command-Header With Argument Shorthand
```
\cmd: …
```

### Begin Indented Body
```
|cmd:
    …
    …
```

### Begin Enumerated List
```
|cmd:
    * Hello World
    * Hello World
```


# Section Syntax

## Implicit Section-Header Syntax
```
\\section\\
||sub-section||
\\section of type as subtype\\
||sub-section of type as subtype||
```

```
\\section.subsection\\
||subsection.subsection||
```

## Implicit Section Body Types

### Begin Enumerated List
```
\\section\\
    * …
    * …
    * …
||sub-section||
    * …
    * …
    * …
```

```
\\section\\
    [label] …
    [label] …
    [label] …
||sub-section||
    [label] …
    [label] …
    [label] …
```

- With Pipe Commands
```
\\section\\
|section-cmd
||sub-section||
|sub-section-cmd
```


## Explicit Section Syntax
```
<section>…</section>
<section of type as subtype>…</section>
<section key=value>…</section>
```

### Running Server

## Build
    Run: cargo build

## Get weather success
    Run: ./target/debug/weather Hanoi
    Result:
        +-----------------+-----------------+
        |      Weather in Hanoi Today!      |
        +-----------------+-----------------+
        | Temperature     | 28              |
        +-----------------+-----------------+
        | Feels like      | 32.25           |
        +-----------------+-----------------+
        | Temperature Min | 28              |
        +-----------------+-----------------+
        | Temperature Max | 28              |
        +-----------------+-----------------+
        | Pressure        | 1009            |
        +-----------------+-----------------+
        | humidity        | 81              |
        +-----------------+-----------------+
        | Description     | overcast clouds |
        +-----------------+-----------------+

## Get weather error
    Run: ./target/debug/weather Hanoi123
    Restul: Request result:city not found


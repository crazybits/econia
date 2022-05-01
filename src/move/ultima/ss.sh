# Shell scripts for common developer workflows

if [ $1 = cl ]
then # Clean up for sandbox development
    move sandbox clean
    python ../../python/ultima/build.py prep short ../../..
elif [ $1 = pb ]
then # Cargo build and publish bytecode for all modules
    python ../../python/ultima/build.py prep long ../../..
    cargo run -- sources
    python ../../python/ultima/build.py publish ../../../.secrets/f112ce1fb887b85dfef24068aff97749ad148989bd1a2ff8950206586e72a272.key ../../../
elif [ $1 = na ]
then # Generate new dev account
    python ../../python/ultima/build.py gen  ../../..
elif [ $1 = mb ]
then # Clean up and run move package build
    python ../../python/ultima/build.py prep short ../../..
    move package build
elif [ $1 = tc ]
then # Move package test with coverage, passing optional arguments
    python ../../python/ultima/build.py prep short ../../..
    move package test --coverage $2 $3
elif [ $1 = cs ]
then # Move package coverage summary
    move package coverage summary
elif [ $1 = sa ]
then # Switch Move.toml to use short addresses
    python ../../python/ultima/build.py prep short ../../..
elif [ $1 = ur ]
then # Change directory to Ultima project respository root
    cd ../../../
else
    echo Invalid option
fi
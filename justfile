s1 := "first_2023_backend"
s2 := "second_2024_backend"
user := env_var('USER')
default:
    @just --list

new number name season=s2:
    cargo new {{justfile_directory()}}/{{season}}/{{number}}-{{name}} --name {{name}} && touch {{justfile_directory()}}/{{season}}/{{number}}-{{name}}/README.md



# If the user wants to generate a template straight into the current folder,
# without creating a subfolder for the contents
# and without attempting to initialize a .git repo or similar, the --init flag can be used.
# cargo generate --init --git https://github.com/username-on-github/mytemplate.git

# using local template
# git clone https://github.com/username-on-github/mytemplate.git $HOME/mytemplate # Clone any template
# cargo generate --path $HOME/mytemplate # Use it locally

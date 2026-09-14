# benchmark script

use std/bench
do -i { rm ~/.local/share/zoxide/db.zo ~/.local/share/zdir/zdir.db }

let foo_dir = "/tmp/foo-bar-baz"
mkdir $foo_dir

# MARK: zoxide benchmark
print "==== zoxide benchmark"
for i in [1..13] {
  zoxide add (mktemp -d)
  zoxide add $foo_dir
}

print (bench { zoxide query foo b ba })

# MARK: zdir benchmark
print "==== zdir benchmark"
for i in [1..13] {
  zdir (mktemp) (mktemp -d)
  zdir (mktemp) $foo_dir
}
zdir (mktemp) $foo_dir

print (bench { zdir (mktemp) foo b ba })

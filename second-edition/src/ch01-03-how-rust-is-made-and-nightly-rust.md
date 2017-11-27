# How Rust is made and "nightly Rust"

Before we dive into the language itself, we'd like to finish up the
introductory chapter by talking about how Rust is made, and how that affects
you as a Rust developer. Feel free to skip this section and come back to it
after you've learned the language if you'd like!

## Stability without stagnation

As a language, Rust cares a *lot* about the stability of your code. We want
Rust to be a rock-solid foundation that you can build on, and if things were
constantly changing, that would be impossible. At the same time, if we cannot
experiment with new features, we may not find out important flaws until after
their release, when we can no longer change things.

Our solution to this problem is what we call "stability without stagnation";
that is, the way we can change and improve Rust while making sure that for
our users, things stay nice, stable, and boring.

Our guiding principle for Rust releases is this: you should never have to fear
upgrading to a new version of stable Rust. Each upgrade should be painless.
At the same time, the upgrade should bring you new features, less bugs, and
faster compile times.

## Choo, choo! Release channels and riding the trains

Rust development operates on a *train schedule*. That is, all development is
done on the `master` branch of the Rust repository, and releases follow "the
train model." There are three *release channels* for Rust:

* Nightly
* Beta
* Stable

Most Rust developers primarily use Rust Stable, but those who want to try
out experimental new features may use nightly or beta. Here's an example of
how this works: let's assume that the Rust team is working on the release of
Rust 1.5. That release happened in December of 2015, but it will provide us
with realistic version numbers. A new feature is added to Rust: a new commit
lands on the `master` branch. Each night, a new nightly version of Rust is
produced. Every day is a release day, and these releases are created by our
release infrastructure automatically. So as time passes, our releases look
like this, once a night:

```text
nightly: * - - * - - *
```

Every six weeks, it's time to prepare a new release! The `beta` branch of
the Rust repository branches off from the `master` branch used by nightly.
Now, there are two releases:

```text
nightly: * - - * - - *
                     |
beta:                *
```

Most Rust users do not use beta actively, but test against beta in their CI
system to help test against regressions. In the meantime, there's still a nightly
release every night:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Let's say a regression is found. Good thing we had some time on beta before it
snuck into a release! The fix is applied to `master`, so that nightly is fixed,
and then the fix is backported to the beta branch, and a new release of beta
is produced:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

Six weeks after the first beta was created, it's time for a stable release! The
`stable` branch is produced from the `beta` branch:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Hooray! Rust 1.5 is done! However, we've forgotten one thing: since the
six weeks have gone by, we also need a new beta of the *next* version of
Rust, 1.6. So after `stable` branches off of `beta`, the next version of
`beta` branches off of `nightly` again:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

This is called the "train model" because every six weeks, the release "leaves
the station", but still has to take a journey before it arrives.

This also means that Rust releases every six weeks, like clockwork. If you
know the date of one Rust release, you can know the date of the next one:
it's six weeks later.

Thanks to this process, you can always check out the next build of Rust and
verify for yourself that it's easy to upgrade to: if something breaks, you
can report it to the team and get it fixed before the next release happens!
This is relatively rare, but `rustc` is still a piece of software, and
bugs do exist.

## Unstable features

There's one more catch with this release model: unstable features. Rust uses
a technique called "feature flags" to determine what features are enabled in
a given release. If a new feature is under active development, it lands on
`master`, and therefore, in nightly, but behind a *feature flag*. If you
as a user wish to try out the work-in-progress feature, you can, but you
must annotate your source code with the appropriate flag to opt in.

There's one more piece to this puzzle: if you're using a beta or stable
release of Rust, you cannot use any feature flags. This is the key that
allows us to get practical use with new features before we declare them
stable forever. Those who wish to opt into the bleeding edge can do so,
and those who want a rock-solid experience can stick with stable and know
that their code won't break. Stability without stagnation.

This book only contains information about stable features, as in-progress
features are still changing, and surely they'll be different between when
this book was written and when they were enabled in stable builds. You
can find documentation for nightly-only features online.

## Rustup and the role of Rust nightly

Rustup makes it easy to change between different release channels of Rust,
on a global or per-project basis. By default, you'll have stable Rust
installed. To install nightly, for example:

```bash
$ rustup install nightly
```

You can see all of the toolchains you have installed with `rustup` as well. Here's
an example on one of your authors' computers:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

As you can see, the stable toolchain is the default. Most Rust users use stable
most of the time. But maybe you want to use stable most of the time, but use
nightly on a specific project, because you care about a cutting-edge feature.
To do so, you can use `rustup override`:

```bash
$ cd ~/projects/needs-nightly
$ rustup override add nightly
```

Now, every time you call `rustc` or `cargo` inside of
`~/projects/needs-nightly`, `rustup` will make sure that you are using
nightly Rust, rather than the default of stable. This comes in handy when you
have a lot of Rust projects!

## The RFC process and teams

So how do you learn about these new features? Rust's development model
follows *the RFC process*. If you'd like an improvement in Rust, you can
write up a proposal, called an RFC. This stands for "request for coments",
and when you submit one, you'll get many.

Anyone can write RFCs to improve Rust, and the proposals are reviewed and
discussed by the Rust team, which is comprised of many individual teams.
There's a full list on Rust's website, but there are teams for each area of
the project: language design, compiler implementation, infrastructure,
documentation, and more. The appropriate team reads the comments, writes some
of their own, and eventually, there's consensus to accept or reject the
feature.

If the feature is accepted, an issue is opened on the Rust repository, and
someone can implement it. The person who implements it very well may not be
the person who proposed the feature in the first place! When the
implementation is ready, it lands on the `master` branch behind a feature
gate, as we discussed above.

After some time, once nightly developers have been able to actually try out
the new feature, team members will discuss the feature, how it's worked out
on nightly, and decide if it should make it into stable Rust or not. If the
decision is to move forward, the feature gate is removed, and the feature is
now considered stable! It rides the trains into a new stable release of Rust.
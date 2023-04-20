# IC updater

A tool to generate [AppSRE schedules](https://github.com/app-sre/qontract-schemas/blob/main/schemas/app-sre/schedule-1.yml).

## Usage

Build the binary and copy to the current dir:
```
cargo build --release && cp target/debug/ic_updater .
alias ic_updater=./ic_updater
```

You need to specify some parameters:

```
❯ ic_updater --help
A tool to generate AppSRE schedules

Usage: ic_updater [OPTIONS] --name <NAME> --start-date <START_DATE> --end-date <END_DATE>

Options:
  -n, --name <NAME>                Name of yaml schema. F.e: MY_TEAM-ic-schedule
  -d, --description <DESCRIPTION>  Description of the yaml schema. F.e: MY_TEAM Interrupt Catcher schedule [default: ]
  -u, --users <USERS>...           List of users. F.e: user-a,user-b,user-c
  -l, --length <LENGTH>            Call duty length (in days) [default: 7]
  -s, --start-date <START_DATE>    Starting date (YYYY-MM-DD hh:mm)
  -e, --end-date <END_DATE>        Ending date (YYYY-MM-DD hh:mm)
  -h, --help                       Print help
```

and you can then generate a schedule file:

```
❯ ic_updater --name test -d "test description" --users user-a,user-b -l 3 -s "2022-07-01 07:00" -e "2022-07-07 14:00"
---
# This file was generated using https://github.com/juandspy/ic-updater

$schema: /app-sre/schedule-1.yml

labels: {}

name: test
description: test description
schedule:
- start: '2022-07-01 07:00'
  end: '2022-07-04 07:00'
  users:
  - $ref: user-a
- start: '2022-07-04 07:00'
  end: '2022-07-07 07:00'
  users:
  - $ref: user-b
```

Note that the generated file is logged into stdout. If you want to store the 
output in a file just pipe it: 

```
❯ ic_updater --name test -d "test description" --users user-a,user-b -l 3 -s "2022-07-01 07:00" -e "2022-07-07 14:00" > test.yaml
```

## Errors

There are some inputs that generate an error:

- Not enough users:
```
❯ ic_updater -n test -s "2022-07-01 07:00" -e "2022-07-07 14:00"
ERROR: at least 1 user is needed
```
- End date is not great enough because the length of the schedule is lower than the difference between start and end dates:
```
❯ ic_updater -n test -u a,b -l 3 -s "2022-07-01 07:00" -e "2022-07-02 14:00"
ERROR: end date '2022-07-02 14:00:00' should be after start date + length of schedule, which is '2022-07-04 07:00:00'
```
- End date is before start date:
```
❯ ic_updater --name test -d "test description" --users user-a,user-b -l 3 -s "2022-07-01 07:00" -e "2021-07-07 14:00"
ERROR: end date '2021-07-07 14:00:00' should be greater than date '2022-07-01 07:00:00'
```

##  TODO

1. Support a diffent hour in the end date. For example:
```yaml
---
$schema: /app-sre/schedule-1.yml

labels: {}

name: SCHEDULE-NAME
description: SCHEDULE_DESCRIPTION

schedule:
- start: '2022-07-01 07:00'
  end: '2022-07-04 14:00'
  users:
  - $ref: /teams/TEST_TEAM/users/USER_A.yml
- start: '2022-07-04 07:00'
  end: '2022-07-07 14:00'
  users:
  - $ref: /teams/TEST_TEAM/users/USER_B.yml
```
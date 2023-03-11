**msg.trinket**

===

**_one message to rule them all_**

only the latest message will be shown. It can be overwritten to anything anyone wants, whenever.

===

**usage**


get the latest msg:

```bash
curl msg.trinket.icu
```

set the latest msg:

```bash
curl msg.trinket.icu -d "<your message>"
```

or

```bash
cat <myfile> | curl msg.trinket.icu --data-binary @-
```

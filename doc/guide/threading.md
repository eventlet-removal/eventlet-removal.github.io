---
layout: base.html
title: Migrating to threading
permalink: /guide/migrating-to-threading/
---

## Practical Guide: Migrating from Eventlet to Threading

This document propose patterns and best practices identified
to migrate code using **Eventlet** to **threading**.

### Replacing Imports

**Before:**
```python
import eventlet
from eventlet import semaphore, event
```

**After:**
```python
import threading
```

---

### 1. Replacing Green Threads

**Eventlet:**
```python
eventlet.spawn(function)
```

**Threading:**
```python
threading.Thread(target=function).start()
```

**Eventlet with delay:**
```python
eventlet.spawn_after(delay, function)
```

**Threading:**
```python
threading.Timer(delay, function).start()
```

---

### 2. Replacing Timeout Handling

**Eventlet:**
```python
from eventlet import timeout

with timeout.Timeout(seconds=timeout_value):
    # action
```

**Threading (manual handling):**
```python
thread = threading.Thread(target=function)
thread.start()
thread.join(timeout=timeout_value)

if thread.is_alive():
    LOG.warning("Action timed out, waiting for the thread to finish...")
    thread.join()  # extended waiting if necessary
```

---

### 3. Replacing Semaphores and Events

**Eventlet:**
```python
from eventlet import semaphore, event

lock = semaphore.Semaphore()
flag = event.Event()
```

**Threading:**
```python
lock = threading.Semaphore()
flag = threading.Event()
```

---

### 4. Replacing Sleep Calls

**Eventlet:**
```python
eventlet.sleep(1)
```

**Threading:**
```python
import time

# sleep 1 second
time.sleep(1)
```

---

### 5. Explicit Thread Termination

**Eventlet:**
```python
thread.wait()
thread.kill()
```

**Threading (explicit wait):**
```python
thread.join()
```

---

### 6. Migrating Executors

**Before (with Eventlet):** Utilized greenthreads, easy to manage but dependent on Eventlet monkeypatching.

**After (Threading):** Explicit shift to native OS threads, typically via `threading.Thread` or `concurrent.futures.ThreadPoolExecutor`.

Concrete example:

**Eventlet:**
```python
from futurist import GreenThreadPoolExecutor
executor = GreenThreadPoolExecutor()
```

**Threading:**
```python
from concurrent.futures import ThreadPoolExecutor
executor = ThreadPoolExecutor(max_workers=10)
```

#### Best practices for Executors:
- Explicitly monitor active threads.
- Proactively handle timeouts and logging.
- Remember that native threads cannot be directly terminated, making careful lifecycle management essential.

---

### 7. Additional Best Practices

- **Active Thread Monitoring:** Explicitly log threads exceeding expected runtimes.
- **Proactive Handling of Blocking Threads:** Always implement mechanisms to detect blocked threads.

```python
if thread.is_alive():
    LOG.warning("The action thread has exceeded the expected runtime.")
```

---

### Summary of Benefits
- Elimination of monkey patching (simplified code)
- Improved compatibility with modern libraries
- Reduced risk of unexpected behaviors associated with greenthreads

This guide will simplify your migration from Eventlet to Threading, ensuring
robust and straightforward thread management.

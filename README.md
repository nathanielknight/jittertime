When scheduling recurring automated tasks (e.g. web scraping, backups, restarts, etc.) it's often useful to *think* about the task as being scheduled at a round time (e.g. midnight) but to *actually* schedule the task slightly after that time to avoid accidentally coordinating with a bunch of other processes to overload a system.

To avoid this problem, this project gives you a little UI to help randomly choose a time slightly after a given time.


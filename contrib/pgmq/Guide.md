# What is this extension?

> Lightweight message queue.

`pgmq` is a rust-based extension designed to implement message queues. It uses technologies also associated with Kafka, RabbitMQ, and SQS.

# When should you use it?

This extension can be used when you’re interested in managing operational pipelines, especially in asynchronous tasks and distributed systems. It is also worth considering when you need features like guaranteed exactly-once delivery, optional archival for replay-ability, and the familiarity of SQL for managing your queues.

# Example use case.

A newly-founded Postgres startup requires a job queue to manage tasks between their control-plane and data-plane within their managed cloud offering. They needed operations like "create postgres cluster" and "update postgres cluster" to be queued and executed seamlessly. To simplify their architecture and reduce technology sprawl, they implemented `pgmq`, creating an efficient message queue system directly within Postgres.

# Example test script.

```
-- Creating a new queue
SELECT pgmq.create('my_queue');

-- Sending messages to the queue
SELECT * FROM pgmq.send('my_queue', '{"foo": "bar1"}');
SELECT * FROM pgmq.send('my_queue', '{"foo": "bar2"}');

-- Reading messages from the queue and making them invisible for a duration
SELECT * FROM pgmq.read('my_queue', 30, 2);

-- If the queue is empty or all messages are invisible, it returns no rows
SELECT * FROM pgmq.read('my_queue', 30, 1);

-- Archiving a message
SELECT * FROM pgmq.archive('my_queue', 2);

-- Viewing the archived message
SELECT * FROM pgmq.a_my_queue;

-- Sending another message and then deleting it
SELECT pgmq.send('my_queue', '{"foo": "bar3"}');
SELECT pgmq.delete('my_queue', 3);

-- Drop queue(s) to complete example
SELECT pgmq.drop_queue('my_queue');
```

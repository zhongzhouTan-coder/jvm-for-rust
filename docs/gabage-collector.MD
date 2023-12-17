# Garbage Collector Design

## Introduction

Garbage Collector is a program that manages memory automatically. It is a part of the runtime system. It is
responsible for allocating and freeing memory.

## Requirements Analysis

- Java Object allocation [small object is the most object]
- Java Object lifetime [short-lived object is the most object]

## Implementation - Immix Garbage Collector

### Overview

Immix is a region-based garbage collector. It divides the heap into fixed-size regions.

### Allocation

- Thread-local allocators are used to allocate objects, when there is not enough memory, the thread-local allocator will request a new region from the global allocator.
- Global allocator is used to allocate block of memory to thread-local allocator. It will request a new block from the OS when there is not enough memory.

### Collection

- trigger: when global allocator is not able to allocate a new block because of memory limit is reached, it will trigger a garbage collection.
- phase: mark, sweep with opportunistic evacuation

### Data structure design

1. Global Allocator [Synchronized]
  `freeBlockList`: a linked list to store block of memory that is free
  `committedWordSize`: committed memory word size that has been required from operating system
  `limitedWordSize`: the limit of memory word size that can be requested from operating system
  `require_block()`: return a block of memory that is free, and can be called by thread-local allocator
  `collect()`: collect garbage, can be triggered when there is not enough memory to allocate a new block
  `mark()`: iterate from the application roots and mark live objects
  `sweep()`: iterate to all thread-local allocators and sweep dead objects
2. Thread-local Allocator [Unsynchronized]
  `UsedBlockList`: a linked list to store block of memory that allocated to thread-local allocator
  `freeBlockList`: a linked list to store block of memory that is free, can be return to global allocator
  `headRoom`: a several block of memory that is reserved for opportunistic evacuation and should never return to global allocator
  `bmpCursor`: a cursor to indicate the start address of the next free hole in the block
  `bmpLimit`: a address to indicate the end address of the block
  `-allocate(wordSize)`: allocate a word size of memory and return the start address that can be used for object allocation
  `free()`: free the current thread local allocator and return its all memory to global allocator
3. Block
  `blockHeader`: a block header that include block mark and line mark table
  `baseAddress`: the start address of the block memory
  `mark_free()`: mark this block to free
  `mark_recyclable()`: mark this block to recyclable when the block has at least one hole
  `mark_line(address)`: mark the line that the address belongs to
4. BlocKHeader
  `blockMark`: indicate the block of state that may be free, recyclable or unavailable
  `lineMarkTable`: the line mark table that record the state of each line in the block
5. BlockMark - Enum
6. LineMarkTable
  `baseAddress`: the start address of the line mark table
7. LineMark - Enum

### Algorithm

1. *object allocation*
  when java application allocate a new object, it will call thread-local allocator to allocate memory for it. The thread-local allocator will return a start address that can be used for object allocation. But if there is not have
  enough memory a garbage collection will be triggered.
2. *garbage collection*
  when a garbage collection is triggered, the global first iterate from the application roots and mark live objects.
  Then it will iterate to all thread-local allocators and mark block state and reset line state and return free block to global allocator, and also, it will do opportunistic evacuation to do compaction.

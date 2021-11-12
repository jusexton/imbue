# Imbue

![](https://github.com/jsextonn/imbue/workflows/build/badge.svg)

Lightning quick dataset imbuing framework for making sure your data is ready for presentation!

## Why

Data is considered missing when an x value does not exist between the smallest and largest given data points. Imbue is
able to fill in the missing values given some dataset.

## How it Works

1. Provide the dataset that needs to be "imbued" and fill in the missing data.
2. Provide a strategy on how the missing data should be populated.

```json
{
  "strategy": "zeroed | last_known | average",
  "dataset": [
    {
      "x": 0,
      "y": 5.5
    },
    {
      "x": 4,
      "y": 12
    },
    {
      "x": 7,
      "y": 54.72
    }
  ]
}
```

## Strategies

### Zeroed

Creates missing data points with 0 y value.

Input:

```json
{
  "strategy": "zeroed",
  "dataset": [
    {
      "x": 1,
      "y": 1
    },
    {
      "x": 5,
      "y": 10
    }
  ]
}
```

Output:

```json
{
  "dataset": [
    {
      "x": 2.0,
      "y": 0.0
    },
    {
      "x": 3.0,
      "y": 0.0
    },
    {
      "x": 4.0,
      "y": 0.0
    }
  ]
}
```

### Last Known

Creates missing data points with the last known y value.

Input:

```json
{
  "strategy": "last_known",
  "dataset": [
    {
      "x": 1,
      "y": 1
    },
    {
      "x": 5,
      "y": 10
    }
  ]
}
```

Output:

```json
{
  "dataset": [
    {
      "x": 2.0,
      "y": 1.0
    },
    {
      "x": 3.0,
      "y": 1.0
    },
    {
      "x": 4.0,
      "y": 1.0
    }
  ]
}
```

### Average

Creates missing data points with an average y value between the last known data point and the next known data point. If
multiple points are missing, the y values will be adjusted to smoothly approach the next known y value.

Input:

```json
{
  "strategy": "average",
  "dataset": [
    {
      "x": 1,
      "y": 1
    },
    {
      "x": 5,
      "y": 10
    }
  ]
}
```

Output:

```json
{
  "dataset": [
    {
      "x": 2.0,
      "y": 3.25
    },
    {
      "x": 3.0,
      "y": 5.5
    },
    {
      "x": 4.0,
      "y": 7.75
    }
  ]
}
```

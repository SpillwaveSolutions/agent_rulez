# Python Coding Standards

Follow these conventions when writing or editing Python code:

## Code Style
- Follow PEP 8 guidelines
- Use 4 spaces for indentation (no tabs)
- Maximum line length: 88 characters (Black default)
- Use snake_case for functions and variables
- Use PascalCase for classes
- Use UPPER_SNAKE_CASE for constants

## Type Hints
- Add type hints to all function parameters and return values
- Use `Optional[T]` for nullable types
- Use `list[T]`, `dict[K, V]` (Python 3.9+) instead of `List`, `Dict`
- Import types from `typing` when needed

## Documentation
- Add docstrings to all public functions, classes, and modules
- Use Google-style or NumPy-style docstrings consistently
- Include parameter descriptions and return value documentation

## Error Handling
- Use specific exception types, not bare `except:`
- Create custom exceptions for domain-specific errors
- Include meaningful error messages

## Testing
- Write tests in `tests/` directory mirroring `src/` structure
- Use pytest for testing
- Aim for >80% code coverage on new code
- Name test files with `test_` prefix

## Example

```python
from typing import Optional

def calculate_total(
    items: list[dict[str, float]],
    discount: Optional[float] = None,
) -> float:
    """Calculate the total price of items with optional discount.
    
    Args:
        items: List of items with 'price' and 'quantity' keys.
        discount: Optional discount percentage (0-100).
    
    Returns:
        Total price after applying discount.
    
    Raises:
        ValueError: If discount is not between 0 and 100.
    """
    if discount is not None and not 0 <= discount <= 100:
        raise ValueError(f"Discount must be 0-100, got {discount}")
    
    total = sum(item["price"] * item["quantity"] for item in items)
    
    if discount:
        total *= (1 - discount / 100)
    
    return total
```

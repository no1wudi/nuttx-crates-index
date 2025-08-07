<template>
  <div class="filters-row">
    <div class="filter-container">
      <label for="crate-filter">Filter by crate name: </label>
      <input
        id="crate-filter"
        type="text"
        :value="crateFilter"
        @input="$emit('update:crateFilter', $event.target.value)"
        placeholder="Enter crate name..."
      />
    </div>

    <div class="test-state-filters">
      <label>Filter by test state: </label>
      <button
        :class="['filter-btn', testStateFilter === 'all' ? 'active' : '']"
        @click="$emit('update:testStateFilter', 'all')"
      >
        All
      </button>
      <button
        :class="['filter-btn', testStateFilter === 'pass' ? 'active' : '']"
        @click="$emit('update:testStateFilter', 'pass')"
      >
        ✅ Pass
      </button>
      <button
        :class="['filter-btn', testStateFilter === 'fail' ? 'active' : '']"
        @click="$emit('update:testStateFilter', 'fail')"
      >
        ❌ Fail
      </button>
      <button
        :class="['filter-btn', testStateFilter === 'skip' ? 'active' : '']"
        @click="$emit('update:testStateFilter', 'skip')"
      >
        ⏭️ Skip
      </button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Filters',
  props: {
    crateFilter: {
      type: String,
      required: true
    },
    testStateFilter: {
      type: String,
      required: true
    }
  },
  emits: ['update:crateFilter', 'update:testStateFilter']
}
</script>

<style scoped>
.filters-row {
  margin: 20px 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 20px;
}

.filter-container {
  display: flex;
  align-items: center;
  gap: 10px;
}

.filter-container input {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  min-width: 250px;
}

.filter-container input:focus {
  outline: none;
  border-color: #4caf50;
  box-shadow: 0 0 5px rgba(76, 175, 80, 0.2);
}

.test-state-filters {
  display: flex;
  align-items: center;
  gap: 10px;
}

.test-state-filters label {
  font-weight: 500;
  color: #333;
}

.filter-btn {
  padding: 8px 16px;
  border: 1px solid #ddd;
  border-radius: 4px;
  background-color: white;
  color: #666;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 5px;
}

.filter-btn:hover {
  border-color: #2196f3;
  color: #2196f3;
}

.filter-btn.active {
  background-color: #2196f3;
  border-color: #2196f3;
  color: white;
}

.filter-btn.active:hover {
  background-color: #1976d2;
  border-color: #1976d2;
}
</style>
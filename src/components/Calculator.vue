<template>
  <div>
    {{ a }} <slot/> {{ b }} = {{ result }}
  </div>
</template>

<script>
import loadWasm from '../lib.rs'

export default {
  name: 'Calculator',
  props: {
    a: {
      type: Number
    },
    b: {
      type: Number
    },
    operation: {
      type: String,
      default: 'add'
    }
  },
  data () {
    return {
      calc: (a, b) => Number()
    }
  },
  computed: {
    result () {
      return this.calc(this.a, this.b)
    }
  },
  beforeCreate () {
    loadWasm()
      .then(result => {
        this.calc = result.instance.exports[this.operation]
      })
      .catch(err => console.error(err))
  }
}
</script>

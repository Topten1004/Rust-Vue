<template>
  <div id="app">
    <img alt="Vue logo" src="./assets/logo.png">
    <Calculator :a="1" :b="1">+</Calculator>
  </div>
</template>

<script>
import loadWasm from './lib.rs'
import Calculator from './components/Calculator.vue'

export default {
  name: 'app',
  components: {
    Calculator
  },
  beforeCreate () {
    loadWasm()
      .then(async (result) => {
        const pointer = result.instance.exports.get_key()
        const stringBuffer = new Uint8Array(result.instance.exports.memory.buffer, pointer, 9)
        console.log(this.fromBuffer(stringBuffer))
        result.instance.exports.free_key(pointer)
      })
      .catch(err => console.error(err))
  },
  methods: {
    fromBuffer (buff) {
      let key = ''
      for (let i = 0; i < buff.length; i++) {
        key += String.fromCharCode(buff[i])
      }
      return key
    }
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>

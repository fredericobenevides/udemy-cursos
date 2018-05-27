<template>
  <div>
    <!-- <p>Counter is: {{ counter }}</p> -->
    <!-- <p>Number of Clicks: {{ clicks }}</p> -->

    <p>Counter is: {{ doubleCounter }}</p>
    <p>Number of Clicks: {{ stringCounter }}</p>
  </div>
</template>

<script>
import { mapGetters } from 'vuex';
import * as types from '../store/types';

export default {
  computed: {
    //...mapGetters(['doubleCounter', 'stringCounter'])
    ...mapGetters({
      doubleCounter: types.DOUBLE_COUNTER,
      stringCounter: types.CLICK_COUNTER
    })
  }

  //computed: {
  //  counter() {
  //    //return this.$store.state.counter * 2;
  //    return this.$store.getters.doubleCounter;
  //  },
  //  clicks() {
  //    return this.$store.getters.stringCounter;
  //  }
  //}
};
</script>

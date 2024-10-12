import React from 'react';
import styled from 'styled-components';
import styles from "./Content.scoped.scss?scopeId=4cfc5488";
import Grid from './Grid';
import GridForwardProps from './GridForwardProps';
const Text = styled.div`
  color: #666;
`;
const FromExample = (props)=>{
    return <div className="content" data-v-4cfc5488="">

      <h3 data-v-4cfc5488="">Styling html tags</h3>

      <p data-v-4cfc5488="">p tag with style</p>



      <React.Fragment>

        <div data-v-4cfc5488="">

          <p data-v-4cfc5488="">content wrapped with React Fragment should be fine</p>

        </div>

      </React.Fragment>



      <h3 data-v-4cfc5488="">Using classes</h3>

      <div className="grid" data-v-4cfc5488=""/>

      <div className="grid" data-v-4cfc5488=""/>



      <h3 data-v-4cfc5488="">Styling child components with css modules</h3>

      <Grid className={styles.grid} data-v-4cfc5488=""/>

      <Grid className={styles.grid} data-v-4cfc5488=""/>



      <h3 data-v-4cfc5488="">

        Styling child components which forward data-v attributes to its root

        element

      </h3>

      <GridForwardProps className="content-grid" data-v-4cfc5488=""/>

      <GridForwardProps className="content-grid" data-v-4cfc5488=""/>



      <h3 data-v-4cfc5488="">Styling with styled-components</h3>

      <Text className="text" data-v-4cfc5488="">Some content in styled-components</Text>

    </div>;
};
export default Content;

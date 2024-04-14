import React from 'react';
import styled from 'styled-components';
import styles from "./Content.scoped.scss?scopeId=e11877f5";
import Grid from './Grid';
import GridForwardProps from './GridForwardProps';
const Text = styled.div`
  color: #666;
`;
const FromExample = (props)=>{
    return <div className="content" data-v-e11877f5="">

      <h3 data-v-e11877f5="">Styling html tags</h3>

      <p data-v-e11877f5="">p tag with style</p>



      <React.Fragment>

        <div>

          <p>content wrapped with React Fragment should be fine</p>

        </div>

      </React.Fragment>



      <h3 data-v-e11877f5="">Using classes</h3>

      <div className="grid" data-v-e11877f5=""/>

      <div className="grid" data-v-e11877f5=""/>



      <h3 data-v-e11877f5="">Styling child components with css modules</h3>

      <Grid className={styles.grid} data-v-e11877f5=""/>

      <Grid className={styles.grid} data-v-e11877f5=""/>



      <h3 data-v-e11877f5="">

        Styling child components which forward data-v attributes to its root

        element

      </h3>

      <GridForwardProps className="content-grid" data-v-e11877f5=""/>

      <GridForwardProps className="content-grid" data-v-e11877f5=""/>



      <h3 data-v-e11877f5="">Styling with styled-components</h3>

      <Text className="text" data-v-e11877f5="">Some content in styled-components</Text>

    </div>;
};
export default Content;

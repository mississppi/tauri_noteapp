import React, { ChangeEvent, ChangeEventHandler } from 'react';
import {invoke} from '@tauri-apps/api/tauri';
import {useEffect, useState, useCallback, useRef} from 'react';

const Main = () => {
    console.log("Main components");
    // const [id, setId] = useState(props.id);
    // const [title, setTitle] = useState(props.title);
    // const [content, setContent] = useState(props.content);
    
    // const onChagneTitle = (e: React.ChangeEvent<HTMLInputElement>) => {
    //     setTitle(e.target.value);
    // };

    // const onChagneContent = (e: ChangeEvent<HTMLTextAreaElement>) => {
    //     setContent(e.target.value);
    // };
    const insert_post = () => {
        invoke('insert', {title: "no title", content:""});
    }

    return (
        <div className="main_area">
            <div className="main_area_inner">
                <div className="title_area">
                    <input 
                        className="title_input" 
                        type="text"
                        name="title"
                    />
                </div>
                <div id="content_area" className="content_area">
                    <textarea
                        id = "content_input"
                        className="content_input"
                        name="content"
                    >
                    </textarea>
                </div>
                <button onClick={(insert_post)}>insert</button>
            </div>
        </div>
    )
}
export default Main
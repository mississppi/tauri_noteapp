const List = (props: any) => {
    console.log("Listコンポ");
    const articles: any[] = [];
    for (const key in props.articles){
        articles.push(
        <li className="list_article" key={key} id={props.articles[key].id} >
            <div className="list_title" key={key} onClick={props.view_article} id={props.articles[key].id}> {props.articles[key].title}</div>
        </li>
        )
    }
    return (
        <div className="list_area">
            <div className="list_inner_area">
                <ul >{articles}</ul>
                {/* <button onClick={props.clear_form}>clear</button> */}
            </div>
        </div>
    )
}
export default List